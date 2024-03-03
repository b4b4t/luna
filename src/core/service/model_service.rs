use std::{
    env,
    fs::{self, OpenOptions},
};
// use chrono::{DateTime, Local};
use super::sqlserver_provider::get_tables;
use crate::{
    core::{
        dao::{model::ModelDao, row::TableRowDao, table::TableDao},
        data::{model_dal::ModelDal, SurrealDb},
        dto::{model::Model, table::Table},
    },
    println_success,
};

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

    pub async fn read_data(db: &SurrealDb) -> anyhow::Result<()> {
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

    pub async fn get_model_list(db: &SurrealDb) -> anyhow::Result<()> {
        println!("Models :");

        let models = ModelDal::get_models(db).await?;
        for model in &models {
            println!("- {}", model.model_name);
        }

        Ok(())
    }

    pub async fn get_models(db: &SurrealDb) -> anyhow::Result<Vec<ModelDao>> {
        ModelDal::get_models(db).await
    }

    pub async fn delete_model(db: &SurrealDb, model_name: &str) -> anyhow::Result<()> {
        // Delete values
        let rows: Vec<TableRowDao> = db
                    .query(
                        "DELETE row where row->owns->table->contains->(model where name = $name) RETURN BEFORE",
                    )
                    .bind(("name", model_name))
                    .await?
                    .take(0)?;
        println_success!("{} rows deleted", rows.len());

        // Delete tables
        let tables: Vec<TableDao> = db
            .query("DELETE table where <-owns<-(model where name = $name) RETURN BEFORE")
            .bind(("name", model_name))
            .await?
            .take(0)?;
        println_success!("{} tables deleted", tables.len());

        // Delete model
        db.query("DELETE model WHERE name = $name")
            .bind(("name", model_name))
            .await?;
        println_success!("Model deleted");

        Ok(())
    }
}
