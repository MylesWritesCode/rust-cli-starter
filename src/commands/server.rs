use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(clap::Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
    args: Option<String>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Start {
        host: Option<String>,
        port: Option<u16>,
    },
}

pub(crate) async fn run(args: &Arguments) -> crate::Result<()> {
    match &args.command {
        Some(commands) => match commands {
            Commands::Start { host, port } => start(host.clone(), *port).await,
        },
        None => start(None, None).await,
    }
}

async fn start(host: Option<String>, port: Option<u16>) -> crate::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "api=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/users", axum::routing::post(create_user));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        host.unwrap_or("0.0.0.0".to_string()),
        port.unwrap_or(3001)
    ))
    .await?;

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    axum::Json(payload): axum::Json<CreateUser>,
) -> impl axum::response::IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (axum::http::StatusCode::CREATED, axum::Json(user))
}

// the input to our `create_user` handler
#[derive(serde::Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(serde::Serialize)]
struct User {
    id: u64,
    username: String,
}
