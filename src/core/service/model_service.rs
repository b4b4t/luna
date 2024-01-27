use std::{
    env,
    fs::{self, OpenOptions},
};

use chrono::{DateTime, Local};
use surrealdb::{engine::local::Db, Surreal};

use crate::{
    core::dto::{Column, ForeignKey, Model},
    println_error, println_success,
    sqlserver::{
        execute_data_query, get_columns, get_tables,
        query_builder::{DataQueryBuilder, Query},
    },
};

pub struct ModelService;

impl ModelService {
    /// Generate a model file with a database
    pub async fn generate_model(model_name: &str) -> anyhow::Result<()> {
        let model_path = env::var("MODEL_PATH").expect("CONNECTION_STRING must be set");

        fs::create_dir_all(&model_path)?;

        let mut model: Model = Model::new(model_name);
        let tables = get_tables().await?;
        for table in tables {
            model.add(table);
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

    pub async fn check_model(model_name: &str) -> anyhow::Result<()> {
        // Get db model
        let db_model = ModelService::build_model_from_database(model_name).await?;
        // Get file model
        let file_model = ModelService::read_model_from_file(model_name)?;

        println!("Checking tables :");
        // Check tables present in the file
        for table in file_model.get_tables_iter() {
            let db_table = db_model.get(table.get_table_name());

            match db_table {
                Some(t) => {
                    let table_name = t.get_table_name();
                    println_success!("{} -> Ok", table_name);

                    // Check columns
                    let columns = table.get_columns_iter();
                    if columns.is_some() {
                        for column in columns.unwrap() {
                            let db_column = table.get_column(column.get_column_name());
                            match db_column {
                                Some(c) => {
                                    println_success!(
                                        "{} -- {} -> Ok",
                                        table_name,
                                        c.get_column_name()
                                    );
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

    /// Read a model from a file
    pub fn read_model_from_file(model_name: &str) -> anyhow::Result<Model> {
        let model_dir_path = env::var("MODEL_PATH").expect("CONNECTION_STRING must be set");
        let model_path = format!("{}/{}.yml", model_dir_path, model_name);
        let file = std::fs::File::open(model_path).expect("Could not open file.");
        let model: Model = serde_yaml::from_reader(file).expect("Could not read values.");

        Ok(model)
    }

    /// Build a model from a database
    pub async fn build_model_from_database(model_name: &str) -> anyhow::Result<Model> {
        let mut tables = get_tables().await?;
        let columns = get_columns().await?;

        // Add columns in tables
        for table in tables.as_mut_slice() {
            let mut t_columns: Vec<Column> = Vec::new();
            for column in &columns {
                if column.get_table_name() == table.get_table_name() {
                    t_columns.push(column.clone());
                }
            }
        }

        let mut model = Model::new(model_name);

        // Add tables in the model
        for table in tables {
            model.add(table);
        }

        Ok(model)
    }

    pub async fn export_data(db: &Surreal<Db>, model: &Model) -> anyhow::Result<()> {
        let current: DateTime<Local> = Local::now();
        let snapshot_name = current.format("%Y-%m-%d_%H:%M:%S").to_string();

        // Save model
        let record: Option<Vec<Model>> = db.create(("model", snapshot_name)).content(model).await?;

        // Get data from model
        for t in model.get_tables_iter() {
            // Create query
            let columns = t
                .get_columns_iter()
                .expect("Columns must be fetched")
                .map(|c| c.get_column_name().to_string())
                .collect::<Vec<String>>();

            let query = DataQueryBuilder::new(t.get_table_name(), &columns).build();

            // Fetch data
            let data = execute_data_query(&query).await?;

            // Save data
        }

        Ok(())
    }
}
