use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ServerArguments {
    #[clap(subcommand)]
    command: Option<ServerCommands>,
    args: Option<String>,
}

#[derive(Subcommand)]
pub enum ServerCommands {
    Start {
        host: Option<String>,
        port: Option<u16>,
    },
}

pub fn server_command(args: ServerArguments) {
    match args.command {
        Some(commands) => match commands {
            ServerCommands::Start { host, port } => start(host, port),
        },
        None => start(None, None),
    }
}

fn start(host: Option<String>, port: Option<u16>) {
    println!("Running the server command from the example module");
}
