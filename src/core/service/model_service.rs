use std::{
    env,
    fs::{self, OpenOptions},
};

use chrono::{DateTime, Local};
use surrealdb::{engine::local::Db, opt::PatchOp, Surreal};

use crate::{
    core::{
        dao::{column::ColumnDao, model::ModelDao},
        dto::{column::Column, model::Model, table::Table},
        service::sqlserver::{
            execute_data_query,
            query_builder::{DataQueryBuilder, Query},
        },
    },
    println_error, println_success, println_warning,
};

use super::sqlserver::{get_columns, get_tables};

pub struct ModelService;

impl ModelService {
    /// Generate a model file with a database
    pub async fn generate_model(model_name: &str) -> anyhow::Result<()> {
        let model_path = env::var("MODEL_PATH").expect("CONNECTION_STRING must be set");

        fs::create_dir_all(&model_path)?;

        let mut model: Model = Model::new(model_name);
        let tables = get_tables().await?;
        for table in tables {
            model.add(Table::from_dao(table));
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}/{}.yml", model_path, model_name))
            .expect("Couldn't open file");

        serde_yaml::to_writer(file, &model).unwrap();

        Ok(())
    }

    /// Export the data of the database into a surrealdb model
    pub async fn export_data(db: &Surreal<Db>, model_name: &str) -> anyhow::Result<()> {
        // Get db model
        let db_model = build_model_from_database(model_name).await?;

        // Get file model
        let mut file_model = read_model_from_file(model_name)?;

        // Check the model
        match check_model(&db_model, &file_model).await {
            Ok(_) => println_success!("Model is ok"),
            Err(err) => println_error!(
                "Error : database and file models are not matching => {}",
                err
            ),
        };

        // Fill columns in the file model
        fill_table_columns(&db_model, &mut file_model);

        // Save model
        let current: DateTime<Local> = Local::now();
        let snapshot_name = current.format("%Y-%m-%d_%H:%M:%S").to_string();
        let model_dao = file_model.to_dao();
        let _: Option<ModelDao> = db
            .create(("model", &snapshot_name))
            .content(model_dao)
            .await?;

        // Get data from model
        println!("Exporting data :");
        // let mut table_idx = 0;
        for t in file_model.get_tables() {
            // Create query
            let columns = t
                .get_columns()
                .expect("Columns must be fetched")
                .iter()
                .map(|c| c.get_column_name().to_string())
                .collect::<Vec<String>>();

            let query = DataQueryBuilder::new(t.get_table_name(), &columns).build();

            // Fetch data
            println!("Fetching data :");
            let rows = execute_data_query(&query).await?;

            // Print values
            // println!("|");
            // for column in columns {
            //     print!(" {: <10} |", column);
            // }
            // for row in &rows {
            //     println!("|");
            //     for row_col in row {
            //         print!(" {: <10} |", row_col);
            //     }
            // }

            // Save data
            println!("Saving {} data in model :", t.get_table_name());
            let patch = PatchOp::add(&format!("/tables/{}/rows", t.get_table_name()), rows);
            let _: Option<ModelDao> = db.update(("model", &snapshot_name)).patch(patch).await?;

            // table_idx += 1;
        }

        println_success!("Data saved successfully");
        Ok(())
    }

    pub async fn read_data(db: &Surreal<Db>) -> anyhow::Result<()> {
        // Get model
        let models: Vec<ModelDao> = db.select("model").await?;

        for model in models {
            println!("Model : {}", model.get_model_name());
            // Get data from model
            for t in model.get_tables_iter() {
                println!("--> Table : {}", t.name);
                // Print columns headers
                println!("|");
                match t.get_columns_iter() {
                    Some(columns) => {
                        for column in columns {
                            print!(" {: <10} |", column.get_column_name());
                        }
                        // Print values
                        match t.get_rows() {
                            Some(rows) => {
                                for row in rows {
                                    println!("|");
                                    for row_col in row {
                                        print!(" {: <10} |", row_col);
                                    }
                                }
                            }
                            None => println!("-- No data found --"),
                        }
                    }
                    None => {
                        print!("No column found |");
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn get_model_list(db: &Surreal<Db>) -> anyhow::Result<()> {
        println!("Models :");
        let models: Vec<ModelDao> = db.select("model").await?;

        for model in models {
            println!("- {}", model.get_model_name());
        }

        Ok(())
    }

    pub async fn delete_model(db: &Surreal<Db>, model_name: &str) -> anyhow::Result<()> {
        db.query("DELETE model WHERE name = $name")
            .bind(("name", model_name))
            .await?;

        Ok(())
    }
}

/// Read a model from a file
fn read_model_from_file(model_name: &str) -> anyhow::Result<Model> {
    let model_dir_path = env::var("MODEL_PATH").expect("CONNECTION_STRING must be set");
    let model_path = format!("{}/{}.yml", model_dir_path, model_name);
    let file = std::fs::File::open(model_path).expect("Could not open file.");
    let model: Model = serde_yaml::from_reader(file).expect("Could not read values.");

    Ok(model)
}

/// Build a model from a database
async fn build_model_from_database(model_name: &str) -> anyhow::Result<ModelDao> {
    let mut tables = get_tables().await?;
    let columns = get_columns().await?;

    // Add columns in tables
    for table in tables.as_mut_slice() {
        let mut t_columns: Vec<ColumnDao> = Vec::new();
        for column in &columns {
            if column.table_name == table.name {
                t_columns.push(column.clone());
                // println!("Table : {}, Col: {}", column.table_name, column.column_name);
            }
        }
        table.add_columns(t_columns);
    }

    // Add tables in a new model
    let mut model = ModelDao::new(model_name);
    for table in tables {
        model.add(table);
    }

    Ok(model)
}

/// Check a model with a database model
async fn check_model(db_model: &ModelDao, file_model: &Model) -> anyhow::Result<()> {
    println!("Checking tables :");
    // Check tables present in the file
    for table in file_model.get_tables() {
        let db_table = db_model.get(table.get_table_name());

        match db_table {
            Some(t) => {
                let table_name = t.name;
                // println_success!("{} -> Ok", table_name);

                // Check columns
                let columns = table.get_columns();
                if columns.is_some() {
                    for column in columns.unwrap() {
                        let db_column = table.get_column(column.get_column_name());
                        match db_column {
                            Some(c) => {
                                println_success!("{} -- {} -> Ok", table_name, c.get_column_name());
                            }
                            None => println_error!(
                                "{} -- {} -> Ko : Missing column in database",
                                table_name,
                                column.get_column_name()
                            ),
                        }
                    }
                }
            }
            None => println_error!(
                "{} -> Ko : Missing table in database",
                table.get_table_name()
            ),
        }
    }

    Ok(())
}

/// Fill a model with a database model
fn fill_table_columns(db_model: &ModelDao, file_model: &mut Model) {
    // Check tables present in the file
    for table in file_model.get_tables_mut() {
        match db_model.get(table.get_table_name()) {
            Some(t) => {
                if t.columns.is_none() {
                    println_warning!("No column found in {}", t.name);
                    return;
                }

                let mut miss_columns = Vec::new();
                // Add missing columns in the model table
                for db_column in t.columns.unwrap().values() {
                    // Find the column in the target model
                    let column = table.get_column(&db_column.column_name);
                    // Add the column if not found in the model
                    match column {
                        Some(c) => {
                            if c.get_type_name() != db_column.type_name {
                                println_warning!(
                                    "Type are different [{}].[{}] - Found : {}, expected : {}",
                                    t.name,
                                    db_column.column_name,
                                    db_column.type_name,
                                    c.get_type_name()
                                );
                            }
                        }
                        None => {
                            // println!("Add missing column {}", db_column.column_name);
                            miss_columns.push(Column::from_dao(db_column));
                        }
                    }
                }
                table.add_columns(miss_columns);
            }
            None => println_error!(
                "{} -> Ko : Missing table in database",
                table.get_table_name()
            ),
        }
    }
}
