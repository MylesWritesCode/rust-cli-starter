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
    if let Some(cmds) = &args.command {
        match cmds {
            ExampleCommands::Example { arg } => example(arg),
        }?;
    };

    Ok(())
}

fn example(arg: &str) -> color_eyre::Result<()> {
    println!("Running example({})", arg);
    Ok(())
}
