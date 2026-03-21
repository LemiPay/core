use crate::data::config::DbConfig;
use crate::data::error::DbError;
use crate::data::pool::{DbConn, DbPool, create_pool};

#[derive(Clone)]
pub struct Db {
    pub(crate) pool: DbPool,
}

impl Db {
    pub fn new(config: DbConfig) -> Self {
        let pool = create_pool(&config.database_url);
        Self { pool }
    }

    pub fn get_conn(&self) -> Result<DbConn, DbError> {
        Ok(self.pool.get()?)
    }
}