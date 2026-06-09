use axum::handler;

pub mod todo {
    pub async fn todo() -> &'static str {
        "Hello to do"
    }
}
