pub trait Route {
    fn routes() -> axum::Router;
}
