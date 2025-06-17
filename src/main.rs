use axum::{Router, routing::get};

async fn homepage() -> &'static str {
    "Welcome to My Rust Website!"
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(homepage));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
