use inquire::Select;

use crate::core::{data::SurrealDb, service::model_service::ModelService};

pub struct DeleteCommand {}

impl DeleteCommand {
    pub async fn run(db: &SurrealDb) -> anyhow::Result<()> {
        let models = ModelService::get_models(db).await?;
        let model_names = models
            .iter()
            .map(|m| &m.model_name)
            .collect::<Vec<&String>>();
        let model = Select::new("Select a model", model_names).prompt();

        match model {
            Ok(choice) => {
                println!("Model {choice} selected");

                ModelService::delete_model(db, choice).await?;
            }
            Err(_) => println!("There was an error, please try again"),
        }

        Ok(())
    }
}
