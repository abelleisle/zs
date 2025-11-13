use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod cmd;
mod config;
mod multiplexer;
mod repo;
mod session;
mod util;
mod workspace;

use config::Config;

#[derive(Parser)]
#[command(name = "zs")]
#[command(about = "Multiplexer session manager", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "s")]
    #[command(about = "Manage sessions")]
    Session {
        #[command(subcommand)]
        command: Option<SessionCommands>,
    },
    #[command(alias = "w")]
    Workspace,
}

#[derive(Subcommand)]
enum SessionCommands {
    #[command(alias = "o")]
    #[command(about = "Open an existing session")]
    Open,

    #[command(alias = "n")]
    #[command(about = "Create a new session")]
    New {
        /// Path to the session directory
        path: PathBuf,
    },

    #[command(alias = "r")]
    #[command(about = "Remove a session")]
    Remove,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    println!("Loaded config from: {}", Config::config_path()?.display());
    println!("Found {} repos", config.repos.len());

    match cli.command {
        None => {
            // Default to session open when no command is provided
            cmd::session::open(&config)?;
        }
        Some(Commands::Session { command }) => match command {
            Some(SessionCommands::Open) | None => {
                cmd::session::open(&config)?;
            }
            Some(SessionCommands::New { path }) => {
                cmd::session::new(&config, path)?;
            }
            Some(SessionCommands::Remove) => {
                cmd::session::remove(&config)?;
            }
        },
        Some(Commands::Workspace) => {
            cmd::workspace::run(&config)?;
        }
    }

    Ok(())
}
