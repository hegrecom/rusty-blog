use core::{
    authorization::RequireAuthorization,
    error::{method_not_allowed_handler, not_found_handler},
};
use std::env;

use admin::controller::admin_controller;
use axum::{
    middleware::{self, from_extractor},
    routing::{get, post, put},
    Router,
};
use post::controller::admin::post_controller as admin_post_controller;
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

    let admin_routes = Router::new()
        .route("/admins/posts", post(admin_post_controller::create))
        .route(
            "/admins/posts/:post_id",
            put(admin_post_controller::update).delete(admin_post_controller::delete),
        )
        .route(
            "/admins/posts/:post_id/publish",
            post(admin_post_controller::publish),
        )
        .route_layer(from_extractor::<RequireAuthorization>());
    let public_routes = Router::new()
        .route("/admins/login", post(admin_controller::login))
        .route("/posts", get(post_controller::index))
        .route("/posts/:post_id", get(post_controller::show));

    let app = admin_routes
        .merge(public_routes)
        .with_state(pool)
        .layer(middleware::from_fn(method_not_allowed_handler))
        .fallback(not_found_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
