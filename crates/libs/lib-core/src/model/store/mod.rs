mod error;

// use std::time::Duration;

pub use self::error::{Error, Result};

use crate::core_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	let max_connections = if cfg!(test) { 1 } else { 5 };

	PgPoolOptions::new()
		.max_connections(max_connections)
		.connect(&core_config().DB_URL)
		.await
		.map_err(|e| Error::FailToCreatePool(e.to_string()))
}
