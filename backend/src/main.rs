use core::{
    authorization::RequireAuthorization,
    error::{method_not_allowed_handler, not_found_handler},
    requesta_id_generator::RequestIdGenerator,
};
use std::env;

use admin::controller::admin_controller;
use axum::{
    http::header,
    middleware::{self, from_extractor},
    routing::{get, post, put},
    Router,
};
use post::controller::admin::post_controller as admin_post_controller;
use post::controller::post_controller;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod admin;
mod core;
mod post;
mod schema;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    initialize_tracing_subscriber_registry();

    let app = admin_routes()
        .merge(public_routes())
        .with_state(database_pool())
        .layer(middleware::from_fn(method_not_allowed_handler))
        .fallback(not_found_handler);
    let app = add_tracing_layer(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn initialize_tracing_subscriber_registry() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rusty_blog_backend=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

fn database_pool() -> deadpool_diesel::postgres::Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let manager =
        deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("Could not create database pool")
}

fn admin_routes() -> Router<deadpool_diesel::postgres::Pool> {
    Router::new()
        .route(
            "/admins/posts",
            get(admin_post_controller::index).post(admin_post_controller::create),
        )
        .route(
            "/admins/posts/:post_id",
            put(admin_post_controller::update)
                .get(admin_post_controller::show)
                .delete(admin_post_controller::delete),
        )
        .route(
            "/admins/posts/:post_id/publish",
            post(admin_post_controller::publish),
        )
        .route_layer(from_extractor::<RequireAuthorization>())
}

fn public_routes() -> Router<deadpool_diesel::postgres::Pool> {
    Router::new()
        .route("/admins/login", post(admin_controller::login))
        .route("/posts", get(post_controller::index))
        .route("/posts/:post_id", get(post_controller::show))
}

fn add_tracing_layer(router: Router) -> Router {
    router
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(SetSensitiveHeadersLayer::new(vec![header::AUTHORIZATION]))
        .layer(SetRequestIdLayer::x_request_id(
            RequestIdGenerator::default(),
        ))
}
