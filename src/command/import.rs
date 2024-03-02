use crate::core::dto::model::Model;
use inquire::Select;

use crate::core::{
    data::SurrealDb,
    service::{import_service::ImportService, model_service::ModelService},
};

pub struct ImportCommand {}

impl ImportCommand {
    pub async fn run(db: &SurrealDb) -> anyhow::Result<()> {
        let models = ModelService::get_models(db)
            .await?
            .iter()
            .map(|m| Model::from_dao(m))
            .collect::<Vec<Model>>();
        let model = Select::new("Select a model", models).prompt();

        match model {
            Ok(selected_model) => {
                println!("Model {selected_model} selected");
                ImportService::import_data(db, selected_model.get_id().unwrap()).await?;
            }
            Err(_) => println!("There was an error, please try again"),
        }

        Ok(())
    }
}
