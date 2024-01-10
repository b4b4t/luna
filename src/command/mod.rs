use clap::{command, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Import,
    Export(ModelArgs),
    Fetch(ModelArgs),
}

#[derive(Args)]
pub struct ModelArgs {
    pub model: Option<String>,
}
