/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
mod commands;
use clap::Parser as _;
use commands::example;

type Result<T> = color_eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(name = "axum starter")]
#[clap(author = "Myles <myles@themapletree.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "Purpose built to start a new project with axum")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Basic command that does things and stuff
    Basic,
    Example(example::Arguments),
}

fn main() -> crate::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    

    if let Some(cmds) = &cli.command {
        match cmds {
            Commands::Basic => basic_command(),
            Commands::Example(args) => example::run(args),
        }?;
    };

    Ok(())
}

fn basic_command() -> crate::Result<()> {
    println!("Running the basic command from the top level");
    Ok(())
}
