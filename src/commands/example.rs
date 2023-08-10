use clap::{Args, Subcommand};

#[derive(Args)]
pub(crate) struct Arguments {
    #[command(subcommand)]
    command: Option<Commands>,
    args: Option<String>,
}

#[derive(Subcommand)]
#[command(arg_required_else_help = true)]
pub(crate) enum Commands {
    /// This will show when calling help on the subcommands (e.g. cargo run -- example help)
    Example(ExampleArguments),
}

pub(crate) fn command(args: &Arguments) -> color_eyre::Result<()> {
    if let Some(cmds) = &args.command {
        match cmds {
            Commands::Example(args) => example(&args),
        }?;
    };

    Ok(())
}

#[derive(Args)]
pub struct ExampleArguments {}

fn example(args: &ExampleArguments) -> color_eyre::Result<()> {
    println!("Running example commands");
    Ok(())
}
