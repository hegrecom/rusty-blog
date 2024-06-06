use crate::{
    admin::{
        dto::{login_request::LoginRequest, token::Token},
        repository::admin_repository::AdminRepository,
    },
    core::error::Error,
};

use super::{jwt_token_service::JwtTokenService, token_service::TokenService};

pub struct AuthenticationService {
    admin_repository: AdminRepository,
}

impl AuthenticationService {
    pub fn new(admin_repository: AdminRepository) -> Self {
        AuthenticationService { admin_repository }
    }
    pub async fn authenticate(&self, login_request: LoginRequest) -> Result<Token, Error> {
        let password = login_request.password();
        let admin = self.admin_repository.fetch_admin().await?;
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
