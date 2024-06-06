use chrono::NaiveDateTime;
use diesel::{
    deserialize::{FromSql, FromSqlRow, Queryable},
    expression::AsExpression,
    Selectable,
};
use serde::Serialize;

use crate::{
    core::into_json::IntoJson,
    schema::{self, posts},
};

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    id: i32,
    title: String,
    content: String,
    status: Status,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, AsExpression, FromSqlRow)]
#[sql_type = "schema::sql_types::Status"]
pub enum Status {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
}

impl FromSql<schema::sql_types::Status, diesel::pg::Pg> for Status {
    fn from_sql(
        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"draft" => Ok(Status::Draft),
            b"published" => Ok(Status::Published),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl IntoJson for Post {}
