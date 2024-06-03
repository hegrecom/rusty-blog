use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    admin::{dto::login_request::LoginRequest, entity::admin::Admin},
    schema::admins,
};

pub async fn login(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    login_request: Result<Json<LoginRequest>, JsonRejection>,
) -> Response {
    match login_request {
        Ok(login_request) => {
            let password = login_request.password();
            let conn = pool.get().await.unwrap();
            let admin = conn
                .interact(|conn| admins::dsl::admins.select(Admin::as_select()).first(conn))
                .await
                .unwrap()
                .unwrap();
            match admin.authenticate(password) {
                Ok(true) => {
                    return (StatusCode::OK, format!("Hello, {}!", password)).into_response()
                }
                Ok(false) => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response(),
                Err(err) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)).into_response()
                }
            }
        }
        Err(err) => (StatusCode::BAD_REQUEST, format!("{:?}", err.body_text())).into_response(),
    }
}
