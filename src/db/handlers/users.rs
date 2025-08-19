use axum::Router;
use axum::routing::{get, post};
use axum::{Json, extract::State};

use crate::ServerConfig;

use crate::db::models::user::CreateUser;

#[axum::debug_handler]
async fn create_account(State(server): State<ServerConfig>, Json(body): Json<CreateUser>) {
    let result = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(body.name)
        .bind(body.email)
        .execute(&server.pool)
        .await
        .map_err(|err: sqlx::Error| err.to_string());
    dbg!(&result);
}

pub fn user_routes() -> Router<ServerConfig> {
    Router::new().route("/", post(create_account))
}
