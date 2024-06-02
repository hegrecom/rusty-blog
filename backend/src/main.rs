use admin::controller::admin_controller;
use axum::{routing::post, Router};

mod admin;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/login", post(admin_controller::login));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
