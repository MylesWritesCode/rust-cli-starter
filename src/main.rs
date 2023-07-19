/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
use clap::{Parser, Subcommand};

mod commands;
use commands::{example};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(author = "Myles <myles@themapletree.io>")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Basic command that does things and stuff
    Basic,
    /// Comments in this position will show in the help text
    Example(example::ExampleArguments),
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(cmd) => match cmd {
            Commands::Basic => basic_command(),
            Commands::Example(args) => example::command(&args),
        },
        None => default_command(),
    }
}

fn default_command() -> color_eyre::Result<()> {
    println!("Running the default command from the top level");
    Ok(())
}

fn basic_command() -> color_eyre::Result<()> {
    println!("Running the basic command from the top level");
    Ok(())
}
