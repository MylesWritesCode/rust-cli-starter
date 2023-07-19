use clap::{Args, Subcommand};

#[derive(Args)]
pub(crate) struct ExampleArguments {
    #[command(subcommand)]
    command: Option<ExampleCommands>,
    args: Option<String>,
}

#[derive(Subcommand)]
#[command(arg_required_else_help = true)]
pub(crate) enum ExampleCommands {
    /// This will show when calling help on the subcommands (e.g. cargo run -- example help)
    Example { arg: String },
}

pub(crate) fn command(args: &ExampleArguments) -> color_eyre::Result<()> {
    match &args.command {
        Some(cmds) => match cmds {
            ExampleCommands::Example { arg } => example(arg),
        },
        None => default(),
    }
}

fn default() -> color_eyre::Result<()> {
    // NOTE This should never run, because of `$[command(arg_required_else_help = true)]`
    println!("Running the default command from the example module");
    Ok(())
}

fn example(arg: &str) -> color_eyre::Result<()> {
    println!("Running example({})", arg);
    Ok(())
}
