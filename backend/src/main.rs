use config::{database, routes};
use core::error::{method_not_allowed_handler, not_found_handler};

use axum::middleware;

mod admin;
mod config;
mod core;
mod post;
mod schema;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    config::tracing::initialize_tracing_subscriber_registry();

    let app = routes::admin_routes()
        .merge(routes::public_routes())
        .with_state(database::database_pool())
        .layer(middleware::from_fn(method_not_allowed_handler))
        .fallback(not_found_handler);
    let app = config::tracing::add_tracing_layer(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
