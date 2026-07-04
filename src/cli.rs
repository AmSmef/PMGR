use core::str;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Get { service: String },
    Add { service: String, username: String, password: String },
    Update { service: String, username: String, password: String },
    Delete { service: String },
}