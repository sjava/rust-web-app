mod error;

pub use self::error::{Error, Result};

use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	PgPoolOptions::new()
		.max_connections(5)
		.connect(&config().DB_URL)
		.await
		.map_err(|e| Error::FailToCreatePool(e.to_string()))
}
