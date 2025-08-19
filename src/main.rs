use std::{env, net::SocketAddr};

use axum::{Router, routing::get};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::db::handlers::users::user_routes;

mod db;

async fn homepage() -> &'static str {
    "Welcome to My Rust Website!"
}

#[derive(Clone)]
struct ServerConfig {
    pub host: String,
    pub port: String,

    pub pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(64)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    dbg!(&pool);
    sqlx::migrate!("db/migrations").run(&pool).await.unwrap();

    let server_config = ServerConfig {
        host: "0.0.0.0".to_string(),
        port: "3000".to_string(),
        pool,
    };
    let addr: SocketAddr = format!("{}:{}", server_config.host, server_config.port)
        .parse()
        .unwrap();

    let api_routes = Router::new().nest("/users", user_routes());

    let router = Router::new()
        .route("/", get(homepage))
        .nest("/api", api_routes)
        .with_state(server_config);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
