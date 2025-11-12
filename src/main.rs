use clap::{Parser, Subcommand};
use anyhow::Result;

mod cmd;
mod config;
mod multiplexer;
mod repo;
mod session;
mod util;

use config::Config;

#[derive(Parser)]
#[command(name = "zs")]
#[command(about = "Multiplexer session manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "s")]
    Session,
    #[command(alias = "w")]
    Workspace,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    println!("Loaded config from: {}", Config::config_path()?.display());
    println!("Found {} repos", config.repos.len());

    match cli.command {
        Some(Commands::Session) | None => {
            cmd::session::run(&config)?;
        }
        Some(Commands::Workspace) => {
            cmd::workspace::run(&config)?;
        }
    }

    Ok(())
}
