use std::net::SocketAddr;

use axum::Router;
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

pub async fn server_command(args: ServerArguments) {
    match args.command {
        Some(commands) => match commands {
            ServerCommands::Start { host, port } => start(host, port).await,
        },
        None => start(None, None).await,
    }
}

async fn start(host: Option<String>, port: Option<u16>) {
    let host = host.unwrap_or("localhost".to_string()).parse().unwrap();
    let port = port.unwrap_or(3003);

    let addr = SocketAddr::new(host, port);

    tracing_subscriber::fmt::init();
    let app = Router::new();

    tracing::debug!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
