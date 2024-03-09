use clap::{command, Args, Parser, Subcommand};

use self::import::ImportArgs;

pub mod delete;
pub mod import;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Import(ImportArgs),
    Export(ModelArgs),
    Fetch(ModelArgs),
    List,
    Delete,
    Read,
}

#[derive(Args)]
pub struct ModelArgs {
    #[arg(short, long)]
    pub model: Option<String>,
    #[arg(short, long)]
    pub take: Option<u64>,
    #[arg(short, long)]
    pub skip: Option<u64>,
}
