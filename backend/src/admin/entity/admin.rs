use diesel::{deserialize::Queryable, Selectable};

use crate::core::error::Error;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::admins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Admin {
    password: String,
}

impl Admin {
    pub fn authenticate(&self, password: &str) -> Result<bool, Error> {
        let result = bcrypt::verify(password, &self.password)
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(result)
    }
}
