use crate::core::{
    dto::model::Model,
    service::{
        export_service::Provider, file_provider::FileProvider,
        sqlserver_provider::provider::SqlServerProvider,
    },
};
use inquire::Select;

use crate::core::{
    data::SurrealDb,
    service::{import_service::ImportService, model_service::ModelService},
};

pub struct ImportCommand {}

impl ImportCommand {
    pub async fn run(db: &SurrealDb, file_name: Option<String>) -> anyhow::Result<()> {
        let models = ModelService::get_models(db)
            .await?
            .iter()
            .map(|m| Model::from_dao(m))
            .collect::<Vec<Model>>();
        let model = Select::new("Select a model", models).prompt();

        match model {
            Ok(selected_model) => {
                println!("Model {selected_model} selected");

                // Select the provider to import the data, file or sql server
                let mut provider: Box<dyn Provider>;

                if file_name.is_some() {
                    provider = Box::new(FileProvider::new(&file_name.unwrap()));
                } else {
                    provider = Box::new(SqlServerProvider::new());
                }

                ImportService::import_data(db, selected_model.get_id().unwrap(), &mut provider)
                    .await?;
            }
            Err(_) => println!("There was an error, please try again"),
        }

        Ok(())
    }
}
