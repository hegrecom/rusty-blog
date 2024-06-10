use axum::{
    middleware::from_extractor,
    routing::{get, post, put},
    Router,
};

use crate::{
    admin::controller::admin_controller,
    core::authorization::RequireAuthorization,
    post::controller::{admin::post_controller as admin_post_controller, post_controller},
};

pub fn admin_routes() -> Router<deadpool_diesel::postgres::Pool> {
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

pub fn public_routes() -> Router<deadpool_diesel::postgres::Pool> {
    Router::new()
        .route("/admins/login", post(admin_controller::login))
        .route("/posts", get(post_controller::index))
        .route("/posts/:post_id", get(post_controller::show))
}
