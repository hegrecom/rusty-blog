use crate::{admin::dto::token::Token, core::error::Error};

pub trait TokenService {
    fn generate_token(&self) -> Result<Token, Error>;
    fn verify_token(&self, token: Token) -> Result<bool, Error>;
}
