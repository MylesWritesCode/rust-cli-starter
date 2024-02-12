use color_eyre::eyre::Ok;

#[derive(clap::Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
    args: Option<String>,
}

// #[derive(clap::Subcommand)]
#[derive(clap::Subcommand)]
pub enum Commands {
    Example {
        arg1: Option<String>,
        arg2: Option<String>,

    },
    ExampleNoArgs,
}

pub(crate) fn run(args: &Arguments) -> crate::Result<()> {
    match &args.command {
        Some(commands) => match commands {
            Commands::Example { arg1, arg2 } => example_fn(arg1, arg2),
            Commands::ExampleNoArgs => example_fn(&None, &None),
        },
        None => example_fn(&None, &None),
    }
}

fn example_fn(_arg1: &Option<String>, _arg2: &Option<String>) -> crate::Result<()> {
    println!("This is an example function");
    Ok(())
}