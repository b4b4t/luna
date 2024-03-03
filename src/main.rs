use clap::Parser;
use command::delete::DeleteCommand;
use command::import::ImportCommand;
use command::{Cli, Commands};
use core::service::export_service::ExportService;
use core::service::model_service::ModelService;
use dotenv::dotenv;
use surrealdb::engine::local::RocksDb;
// use surrealdb::opt::Config;
use surrealdb::Surreal;

mod command;
mod core;

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

    // let db: SurrealDb = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    // db.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await
    // .unwrap();

    db.use_ns("luna").use_db("luna").await?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Import(model_args) => {
            let file_name = model_args.file.clone();
            ImportCommand::run(&db, file_name).await?;
        }
        Commands::Export(model_args) => {
            let model_name = model_args.model.clone().unwrap();
            ExportService::export_data(&db, &model_name).await?;
        }
        Commands::Fetch(model_args) => {
            let model_name = model_args.model.clone();
            ModelService::generate_model(&model_name.unwrap()).await?;
        }
        Commands::List => {
            ModelService::get_model_list(&db).await?;
        }
        Commands::Delete => {
            DeleteCommand::run(&db).await?;
        }
        Commands::Read => {
            // let model_name = model_args.model.clone();
            ModelService::read_data(&db).await?;
        }
    }

    Ok(())
}
