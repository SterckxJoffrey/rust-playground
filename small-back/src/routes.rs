use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(api))
        .route("/todos", get(handlers::todo::todo))
}

async fn api() -> &'static str {
    "Hello world 3"
}
