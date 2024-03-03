use clap::{command, Args, Parser, Subcommand};

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
    Import(ModelArgs),
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
    pub file: Option<String>,
}
