use std::io::Write;

use chrono::NaiveDateTime;
use diesel::{
    deserialize::{FromSql, FromSqlRow, Queryable},
    expression::AsExpression,
    serialize::{IsNull, Output, Result, ToSql},
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

#[derive(Debug, Clone, Copy, Serialize, AsExpression, FromSqlRow)]
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

impl ToSql<schema::sql_types::Status, diesel::pg::Pg> for Status {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> Result {
        match self {
            Status::Draft => out.write_all(b"draft")?,
            Status::Published => out.write_all(b"published")?,
        }
        Ok(IsNull::No)
    }
}

impl IntoJson for Post {}
