use diesel::{prelude::Insertable, query_builder::AsChangeset};
use serde::Deserialize;

use crate::schema::posts;

#[derive(Debug, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = posts)]
pub struct PostRequest {
    title: String,
    content: String,
}
