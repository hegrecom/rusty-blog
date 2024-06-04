use crate::{
    admin::{
        dto::{login_request::LoginRequest, token::Token},
        repository::admin_repository::AdminRepository,
    },
    core::error::Error,
};

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
            .await
            .map_err(|err| Error::InternalServerError(format!("{}", err.to_string())))?;
        match admin.authenticate(password) {
            Ok(true) => Ok(Token::new("Hi".to_string())),
            Ok(false) => Err(Error::Unauthorized),
            Err(err) => Err(Error::InternalServerError(format!("{}", err.to_string()))),
        }
    }
}