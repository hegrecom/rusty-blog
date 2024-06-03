use anyhow::Result;
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};

use crate::{admin::entity::admin::Admin, schema::admins};

pub struct AdminRepository {
    pool: deadpool_diesel::postgres::Pool,
}

impl AdminRepository {
    pub fn new(pool: deadpool_diesel::postgres::Pool) -> Self {
        AdminRepository { pool }
    }

    pub async fn fetch_admin(&self) -> Result<Admin> {
        let conn = self.pool.get().await?;
        let result = conn
            .interact(|conn| admins::table.select(Admin::as_select()).first(conn))
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;

        Ok(result)
    }
}
