use clap::Parser;
use command::{Cli, Commands};
use core::dto::model::Model;
use core::service::model_service::ModelService;
use dotenv::dotenv;
use surrealdb::engine::local::RocksDb;
// use surrealdb::opt::Config;
use surrealdb::Surreal;

mod command;
mod core;
mod sqlserver;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .env variables
    dotenv().ok();

    // Create database connection
    // let config = Config::default().strict();
    // let db: Surreal<surrealdb::engine::local::Db> =
    // Surreal::new::<RocksDb>(("~/luna.db", config)).await?;
    let db: Surreal<surrealdb::engine::local::Db> = Surreal::new::<RocksDb>("~/luna.db").await?;
    // Select a specific namespace / database
    db.use_ns("luna").use_db("luna").await?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Import => {
            println!("Export");
        }
        Commands::Export(model_args) => {
            let model_name = model_args.model.clone().unwrap();
            ModelService::export_data(&db, &model_name).await?;
        }
        Commands::Fetch(model_args) => {
            let model_name = model_args.model.clone();
            ModelService::generate_model(&model_name.unwrap()).await?;
        }
        Commands::List => {
            ModelService::get_model_list(&db).await?;
        }
        Commands::Delete(model_args) => {
            let model_name = model_args.model.clone();
            ModelService::delete_model(&db, &model_name.unwrap()).await?;
        }
        Commands::Read(model_args) => {
            // let model_name = model_args.model.clone();
            ModelService::read_data(&db).await?;
        }
    }

    Ok(())
}
