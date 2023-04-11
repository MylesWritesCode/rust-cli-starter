use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ExampleArguments {
    #[command(subcommand)]
    command: Option<ExampleCommands>,
    args: Option<String>,
}

#[derive(Subcommand)]
pub enum ExampleCommands {
    /// This will show when calling help on the subcommands (e.g. cargo run -- example help)
    Example { arg: String },
}

pub fn example_command(args: &ExampleArguments) -> color_eyre::Result<()> {
    match &args.command {
        Some(commands) => match commands {
            ExampleCommands::Example { arg } => example(arg),
        },
        None => default(),
    }
}

fn default() -> color_eyre::Result<()> {
    println!("Running the default command from the example module");
    Ok(())
}

fn example(arg: &str) -> color_eyre::Result<()> {
    println!("Running example({})", arg);
    Ok(())
}
