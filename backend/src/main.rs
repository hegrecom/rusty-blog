use core::error::not_found_handler;
use std::env;

use admin::controller::admin_controller;
use axum::{
    routing::{post, put},
    Router,
};
use post::controller::post_controller;

mod admin;
mod core;
mod post;
mod schema;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let manager =
        deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("Could not create database pool");

    let app = Router::new()
        .route("/login", post(admin_controller::login))
        .route("/posts", post(post_controller::create))
        .route("/posts/:post_id", put(post_controller::update))
        .with_state(pool)
        .fallback(not_found_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
