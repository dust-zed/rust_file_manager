use anyhow::{Ok, Result};
use clap::Parser;
use commands::{list, stat, rename, Cli, Commands};

mod commands;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List { path } => list::list_files(&path)?,
        Commands::Stat { path } => stat::stat_files(&path)?,
        Commands::Rename { path, prefix } => rename::rename_files(&path, &prefix)?,
    }
    Ok(())
}