use diesel::prelude::Insertable;
use serde::Deserialize;

use crate::schema::posts;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct PostCreation {
    title: String,
    content: String,
}
