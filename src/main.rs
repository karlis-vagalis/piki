use std::env;

use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;

async fn homepage() -> &'static str {
    "Welcome to My Rust Website!"
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(homepage));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(64)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    dbg!(&pool);
    
    sqlx::query(include_str!(r"../db/queries/init/accounts.sql"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(include_str!(r"../db/queries/init/groups.sql"))
        .execute(&pool)
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
