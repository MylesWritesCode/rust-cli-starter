use clap::{Args, Subcommand};

#[derive(Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
    args: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        host: Option<String>,
        port: Option<u16>,
    },
}

pub(crate) fn run(args: &Arguments) -> crate::Result<()> {
    match &args.command {
        Some(commands) => match commands {
            Commands::Start { host, port } => start(host.clone(), *port),
        },
        None => start(None, None),
    }
}

fn start(host: Option<String>, port: Option<u16>) -> crate::Result<()> {
    println!("Running the server command from the example module");
    Ok(())
}
