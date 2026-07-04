mod cli;
mod vault;

use cli::{Cli, Commands};
use vault::{get_entry_data, add_entry_data, update_entry_data, delete_entry_data};
use clap::Parser;
use crate::vault::{VaultEntry, VaultResult};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Get { service } => get_entry_data(&service),
        Commands::Add { service, username, password } => add_entry_data(service, username, password),
        Commands::Update { service, username, password } => update_entry_data(service, username, password),
        Commands::Delete { service } => delete_entry_data(service)
    };

    match result {
        VaultResult::Found(entry) => display_vault_entry(entry),
        VaultResult::Added(entry) => {
            println!("\nAdded entry for {}:", entry.service);
            display_vault_entry(entry);
        },
        VaultResult::Updated(entry) => {
            println!("\nUpdated entry for {}:", entry.service);
            display_vault_entry(entry);
        },
        VaultResult::Deleted(entry) => println!("\nDeleted entry for {} (username: {})\n", entry.service, entry.username.unwrap_or_default()),
        VaultResult::NotFound(service) => println!("\nEntry for {} not found.\n", service),
        VaultResult::AlreadyExists(service) => println!("\nEntry for {} already exists.\nTo update it, try 'pmgr update {}' command.", service, service),
    }   
}

fn display_vault_entry(entry: VaultEntry) {
    match (entry.username, entry.password) {
        (Some(username), Some(password)) => println!("\nUsername: {}\nPassword: {}\n", username, password),
        _ => println!("\nEntry for {} not found.\n", entry.service),
    }
}