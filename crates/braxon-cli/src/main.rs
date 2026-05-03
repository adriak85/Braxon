use clap::{Parser, Subcommand};
use BRAXON_core::BRAXONIdentity;

#[derive(Parser, Debug)]
#[command(name = "Braxon")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Status,
}

fn main() {
    match Cli::parse().command.unwrap_or(Commands::Status) {
        Commands::Status => {
            let id = BRAXONIdentity::current();
            println!("{} {}", id.name, id.version);
        }
    }
}
