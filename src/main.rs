/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
use clap::{Parser, Subcommand};

mod commands;
use commands::example::*;
use commands::server::*;

mod settings;
use settings::Settings;

#[derive(Parser)]
#[clap(name = "axum starter")]
#[clap(author = "Myles <myles@themapletree.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "Purpose built to start a new project with axum")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Basic,
    Example(ExampleArguments),
    Server(ServerArguments)
}

fn main() {
    let cli = Cli::parse();
    let settings = Settings::new();

    match settings {
        Ok(_) => println!("Sucessfully loaded settings!"),
        Err(e) => println!("Error loading settings: {:?}", e),
    }

    match cli.command {
        Some(command) => match command {
            Commands::Basic => basic_command(),
            Commands::Example(args) => example_command(&args),
            Commands::Server(args) => server_command(args),
        },
        None => default_command(),
    }
}

fn default_command() {
    println!("Running the default command from the top level");
}

fn basic_command() {
    println!("Running the basic command from the top level");
}
