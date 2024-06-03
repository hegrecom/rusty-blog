use anyhow::Result;
use diesel::{deserialize::Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::admins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Admin {
    password: String,
}

impl Admin {
    pub fn authenticate(&self, password: &str) -> Result<bool> {
        let result = bcrypt::verify(password, &self.password)?;

        Ok(result)
    }
}
