use self::sqlserver_provider::{get_columns, get_tables};
use super::{
    dao::column::ColumnDao,
    dto::{model::Model, table::Table},
};
use crate::{println_error, println_success, println_warning};
use std::env;

pub mod export_service;
pub mod file_provider;
pub mod import_service;
pub mod model_service;
pub mod sqlserver_provider;

/// Read a model from a file
fn read_model_from_file(model_name: &str) -> anyhow::Result<Model> {
    let model_dir_path = env::var("MODEL_PATH").expect("CONNECTION_STRING must be set");
    let model_path = format!("{}/{}.yml", model_dir_path, model_name);
    let file = std::fs::File::open(model_path).expect("Could not open file.");
    let model: Model = serde_yaml::from_reader(file).expect("Could not read values.");

    Ok(model)
}

/// Build a model from a database
async fn build_model_from_database(model_name: &str) -> anyhow::Result<Model> {
    let mut tables = get_tables().await?;
    let columns = get_columns().await?;

    // Add columns in tables
    for table in tables.as_mut_slice() {
        let mut t_columns: Vec<ColumnDao> = Vec::new();
        for column in &columns {
            if column.table_name == table.name {
                t_columns.push(column.clone());
            }
        }
        table.add_columns(t_columns);
    }

    // Add tables in a new model
    let mut model = Model::new(model_name, model_name);

    for table in tables {
        model.add(Table::from_dao(table));
    }

    Ok(model)
}

/// Check a model with a database model
async fn check_model(db_model: &Model, file_model: &Model) -> anyhow::Result<()> {
    println!("--> Checking tables");
    // Check tables present in the file
    for table in file_model.get_tables() {
        let db_table = db_model.get_table(table.get_table_name());

        match db_table {
            Some(t) => {
                // Check columns
                let columns = table.get_columns();
                if columns.is_some() {
                    for column in columns.unwrap() {
                        let db_column = table.get_column(column.get_column_name());
                        match db_column {
                            Some(c) => {
                                println_success!(
                                    "{} -- {} -> Ok",
                                    t.get_table_name(),
                                    c.get_column_name()
                                );
                            }
                            None => println_error!(
                                "{} -- {} -> Ko : Missing column in database",
                                t.get_table_name(),
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
fn fill_table_columns(db_model: &Model, file_model: &mut Model) {
    // Check tables present in the file
    println!("--> Fetch the columns of each table");
    for table in file_model.get_tables_mut() {
        match db_model.get_table(table.get_table_name()) {
            Some(t) => {
                let colums = t.get_columns();
                if colums.is_none() {
                    println_warning!("No column found in {}", t.get_table_name());
                    return;
                }

                let mut miss_columns = Vec::new();
                // Add missing columns in the model table
                for db_column in colums.unwrap() {
                    // Find the column in the target model
                    let column = table.get_column(&db_column.get_column_name());
                    // Add the column if not found in the model
                    match column {
                        Some(c) => {
                            if c.get_type_name() != db_column.get_type_name() {
                                println_warning!(
                                    "Type are different [{}].[{}] - Found : {}, expected : {}",
                                    t.get_table_name(),
                                    db_column.get_column_name(),
                                    db_column.get_type_name(),
                                    c.get_type_name()
                                );
                            }
                        }
                        None => {
                            // println!("Add missing column {}", db_column.column_name);
                            miss_columns.push(db_column.clone());
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
