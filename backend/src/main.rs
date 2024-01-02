use axum::{
    routing::get,
    Router,
};
use sqlx::postgres::PgPoolOptions;

mod model;

async fn root() -> &'static str {
    "Health check!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
