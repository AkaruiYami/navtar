use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wsp", about = "CLI workspace directory mangement.")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Add { name: String, path: String },
    Get { name: String },
    Remove { name: String },
    List,
}
