use std::{env, net::SocketAddr};

use axum::{extract::State, routing::{get, post}, Json, Router};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

mod db;

async fn homepage() -> &'static str {
    "Welcome to My Rust Website!"
}

async fn get_all_accounts(State(server): State<ServerConfig>) -> Json<Vec<db::models::Account>> {
    let rows = sqlx::query_as!(db::models::Account, "SELECT id, name, type AS \"type: db::models::AccountType\", updated_at, deleted_at FROM accounts")
        .fetch_all(&server.pool)
        .await
        .expect("Unable to get accounts!");
    Json(rows)
}

#[axum::debug_handler]
async fn create_account(
    State(server): State<ServerConfig>,
    Json(body): Json<db::models::CreateAccount>,
) {
    let result = sqlx::query("INSERT INTO accounts (name, type) VALUES ($1, $2)")
        .bind(body.name)
        .bind(body.r#type)
        .execute(&server.pool)
        .await
        .map_err(|err: sqlx::Error| err.to_string());
    dbg!(&result);
}

async fn get_all_groups(State(server): State<ServerConfig>) -> Json<Vec<db::models::Group>> {
    let rows = sqlx::query_as!(db::models::Group, "SELECT * FROM groups")
        .fetch_all(&server.pool)
        .await
        .unwrap();
    Json(rows)
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

    let router = Router::new()
        .route("/", get(homepage))
        .route("/api/accounts", get(get_all_accounts))
        .route("/api/accounts", post(create_account))
        .route("/api/groups", get(get_all_groups))
        .with_state(server_config);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
