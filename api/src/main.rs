use atom_api::prelude::*;
use atom_api::route::Route;
use atom_api::routes::habit::Habit;
use atom_api::state::AppState;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let state = AppState::new(atom_infrastructure::database::database().unwrap());

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(state) //TODO: Send state to actual routes.
        .merge(Habit::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
