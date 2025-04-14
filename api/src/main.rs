use atom_api::error::ApiError;
use atom_api::prelude::*;
use atom_api::routes::habit::Habit;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    tracing_subscriber::fmt::init();

    let client = atom_infrastructure::database::client()?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(Habit::routes(client));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
