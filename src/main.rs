/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
use clap::{Parser, Subcommand};

mod commands;
use commands::example::*;

#[derive(Parser)]
#[clap(name = "Starter kit")]
#[clap(author = "Myles <myles@themapletree.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "This is a starter kit for creating a CLI application with Rust.")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/**
 * These are the subcommands that can be run.
 * @example
 * ```bash
 * $ starter basic
 * $ starter with-args argument
 * ```
 */
#[derive(Subcommand)]
enum Commands {
    Basic,
    // ExampleArguments is defined in the command source file.
    WithArgs(ExampleArguments),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Basic => example_fn(),
        Commands::WithArgs(args) => example_fn_with_args(args),
    }
}
