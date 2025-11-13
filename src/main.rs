use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod cmd;
mod config;
mod features;
mod multiplexer;
mod repo;
mod session;
mod state;
mod util;
mod workspace;

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
    let state = state::State::new();

    match cli.command {
        None => {
            // Default to session open when no command is provided
            cmd::session::open(&state)?;
        }
        Some(Commands::Session { command }) => match command {
            Some(SessionCommands::Open) | None => {
                cmd::session::open(&state)?;
            }
            Some(SessionCommands::New { path }) => {
                cmd::session::new(&state, path)?;
            }
            Some(SessionCommands::Remove) => {
                cmd::session::remove(&state)?;
            }
        },
        Some(Commands::Workspace) => {
            cmd::workspace::run(&state)?;
        }
    }

    Ok(())
}
