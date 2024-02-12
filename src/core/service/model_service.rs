use std::{
    env,
    fs::{self, OpenOptions},
};

// use chrono::{DateTime, Local};
use surrealdb::{engine::local::Db, Surreal};

use crate::core::{
    dao::model::ModelDao,
    dto::{model::Model, table::Table},
};

use super::sqlserver::get_tables;

pub struct ModelService;

impl ModelService {
    /// Generate a model file with a database
    pub async fn generate_model(model_name: &str) -> anyhow::Result<()> {
        let model_path = env::var("MODEL_PATH").expect("CONNECTION_STRING must be set");

        fs::create_dir_all(&model_path)?;

        let mut model: Model = Model::new(model_name, model_name);
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

    pub async fn read_data(db: &Surreal<Db>) -> anyhow::Result<()> {
        // Get model
        let models: Vec<ModelDao> = db.select("model").await?;

        for model in models {
            println!("Model : {}", model.model_name);
            // Get data from model
            // for t in model.get_tables_iter() {
            //     println!("--> Table : {}", t.name);
            //     // Print columns headers
            //     println!("|");
            //     match t.get_columns_iter() {
            //         Some(columns) => {
            //             for column in columns {
            //                 print!(" {: <10} |", column.get_column_name());
            //             }
            //             // Print values
            //             match t.get_rows() {
            //                 Some(rows) => {
            //                     for row in rows {
            //                         println!("|");
            //                         for row_col in row {
            //                             print!(" {: <10} |", row_col);
            //                         }
            //                     }
            //                 }
            //                 None => println!("-- No data found --"),
            //             }
            //         }
            //         None => {
            //             print!("No column found |");
            //         }
            //     }
            // }
        }

        Ok(())
    }

    pub async fn get_model_list(db: &Surreal<Db>) -> anyhow::Result<()> {
        println!("Models :");
        let models: Vec<ModelDao> = db.select("model").await?;

        for model in models {
            println!("- {}", model.model_name);
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
