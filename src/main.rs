use std::env;

use axum::{Router, routing::get};
use sqlx::{postgres::PgPoolOptions, Executor};

async fn homepage() -> &'static str {
    "Welcome to My Rust Website!"
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(homepage));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await.unwrap();

    dbg!(&pool);

    axum::serve(listener, router).await.unwrap();
}
