use diesel::{deserialize::Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::admins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Admin {
    password: String,
}

impl Admin {
    pub fn encrypted_password(&self) -> &str {
        &self.password
    }
}
