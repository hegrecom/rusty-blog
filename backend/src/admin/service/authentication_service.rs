use crate::{
    admin::{
        dto::{login_request::LoginRequest, token::Token},
        repository::admin_repository::AdminRepository,
    },
    core::error::Error,
};

use super::{jwt_token_service::JwtTokenService, token_service::TokenService};

pub struct AuthenticationService {
    pool: deadpool_diesel::postgres::Pool,
}

impl AuthenticationService {
    pub fn new(pool: deadpool_diesel::postgres::Pool) -> Self {
        AuthenticationService { pool }
    }
    pub async fn authenticate(&self, login_request: LoginRequest) -> Result<Token, Error> {
        let password = login_request.password();
        let admin = AdminRepository::new(self.pool.clone())
            .fetch_admin()
            .await?;
        match admin.authenticate(password) {
            Ok(true) => {
                let token = JwtTokenService::new().generate_token()?;
                Ok(token)
            }
            Ok(false) => Err(Error::Unauthorized),
            Err(err) => Err(Error::InternalServerError(err.to_string())),
        }
    }
}
