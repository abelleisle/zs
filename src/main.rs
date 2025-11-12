use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Session) | None => {
            println!("Session command executed!");
        }
        Some(Commands::Workspace) => {
            println!("Workspace command executed!");
        }
    }
}
