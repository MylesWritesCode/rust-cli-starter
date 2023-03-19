/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
use clap::{Parser, Subcommand};

mod commands;
use commands::example::*;

mod settings;
use settings::Settings;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(author = "Myles <myles@themapletree.io>")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Basic command that does things and stuff
    Basic,
    /// Comments in this position will show in the help text
    Example(ExampleArguments),
}

fn main() {
    let cli = Cli::parse();
    let settings = Settings::new();

    match settings {
        Ok(_) => println!("Sucessfully loaded settings!"),
        Err(e) => println!("Error loading settings: {:?}", e),
    }

    match &cli.command {
        Some(command) => match command {
            Commands::Basic => basic_command(),
            Commands::Example(args) => example_command(&args),
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
