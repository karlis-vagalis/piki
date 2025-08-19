use axum::Router;
use axum::routing::{get, post};
use axum::{Json, extract::State};

use crate::ServerConfig;

use crate::db::models::user::{CreateUser, User};

#[axum::debug_handler]
async fn create_user(State(server): State<ServerConfig>, Json(body): Json<CreateUser>) {
    let result = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(body.name)
        .bind(body.email)
        .execute(&server.pool)
        .await
        .map_err(|err: sqlx::Error| err.to_string());
    dbg!(&result);
}

async fn get_users(State(server): State<ServerConfig>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&server.pool)
        .await
        .expect("Failed");
    Json(users)
}

pub fn user_routes() -> Router<ServerConfig> {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_users))
}
