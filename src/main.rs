/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
mod commands;
use clap::Parser as _;
use commands::*;

type Result<T> = color_eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(name = "API starter")]
#[clap(author = "Myles <myles@themapletree.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "A starter project for building APIs with Axum and Rust")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

const SCAFFOLD_ABOUT: &str = "
Scaffolding command for quickly generating new files in your project

This command will not show up in release builds and is only here for your 
convenience during development.";

#[derive(clap::Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Basic command that does things and stuff
    Server(server::Arguments),
    #[cfg(debug_assertions)]
    #[clap(arg_required_else_help = true)]
    #[clap(about = "Scaffolding command for quickly generating new files in your project")]
    #[clap(long_about = SCAFFOLD_ABOUT)]
    Scaffold(scaffold::Arguments),
}

#[tokio::main]
async fn main() -> crate::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    if let Some(cmds) = &cli.command {
        match cmds {
            Commands::Server(args) => server::run(args).await,
            #[cfg(debug_assertions)]
            Commands::Scaffold(args) => scaffold::run(args),
        }?;
    };

    Ok(())
}
