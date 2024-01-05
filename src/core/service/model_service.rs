use std::{
    env,
    fs::{self, OpenOptions},
};

use crate::{
    core::dto::Model,
    sqlserver::{get_columns, get_tables},
};

pub struct ModelService;

impl ModelService {
    /// Generate a model file with a database
    pub async fn generate_model(model_name: String) -> anyhow::Result<()> {
        let model_path = env::var("MODEL_PATH").expect("CONNECTION_STRING must be set");

        fs::create_dir_all(&model_path)?;

        let mut model = Model::new();
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

    pub fn check_model(model_name: String) -> anyhow::Result<()> {
        // Get db model
        let db_model = ModelService::build_model_from_database().await?;
        // Get file model
        let file_model = ModelService::read_model_from_file(model_name).await?;

        // Check tables

        // Check columns

        Ok(())
    }

    /// Read a model from a file
    pub fn read_model_from_file(model_name: String) -> anyhow::Result<Model> {
        let model_dir_path = env::var("MODEL_PATH").expect("CONNECTION_STRING must be set");
        let model_path = format!("{}/{}.yml", model_dir_path, model_name);
        let file = std::fs::File::open(model_path).expect("Could not open file.");
        let model: Model = serde_yaml::from_reader(file).expect("Could not read values.");

        Ok(model)
    }

    /// Build a model from a database
    pub async fn build_model_from_database() -> anyhow::Result<Model> {
        let mut tables = get_tables().await?;
        let columns = get_columns().await?;

        // Add columns in tables
        for column in columns {
            for table in tables.as_mut_slice() {
                if column.get_table_name() == table.get_table_name() {
                    table.add_column(column.clone());
                }
            }
        }

        let mut model = Model::new();

        // Add tables in the model
        for table in tables {
            model.add(table);
        }

        Ok(model)
    }
}
