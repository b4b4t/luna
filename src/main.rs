use crate::sqlserver::{get_columns, get_tables};
use clap::Parser;
use command::{Cli, Commands};
use core::service::model_service::ModelService;
use dotenv::dotenv;
use surrealdb::engine::local::RocksDb;
use surrealdb::opt::Config;
use surrealdb::Surreal;

mod command;
mod core;
mod sqlserver;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // .env variables
    dotenv().ok();

    // Create database connection
    let config = Config::default().strict();
    let db = Surreal::new::<RocksDb>(("luna.db", config)).await?;

    // Select a specific namespace / database
    db.use_ns("luna").use_db("luna").await?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Import => {
            println!("Import");
            get_tables().await?;
            get_columns().await?;
        }
        Commands::Export => {
            println!("Export");
        }
        Commands::Fetch(model_args) => {
            let model_name = model_args.model.clone();
            ModelService::generate_model(model_name.unwrap()).await?;
        }
    }

    Ok(())
}
