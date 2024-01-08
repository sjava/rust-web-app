use crate::ctx::Ctx;
use crate::model::user::{User, UserBmc};
use crate::model::ModelManager;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::info;

type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str =
	"postgres://postgres:postgres@zyb@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
	info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

	let current_dir = std::env::current_dir().unwrap();
	let v: Vec<_> = current_dir.components().collect();
	let path_comp = v.get(v.len().wrapping_sub(3));
	let base_dir = if Some(true) == path_comp.map(|c| c.as_os_str() == "crates") {
		v[..v.len() - 3].iter().collect::<PathBuf>()
	} else {
		current_dir.clone()
	};
	let sql_dir = base_dir.join(SQL_DIR);

	{
		let sql_recreate_db_file = sql_dir.join(SQL_RECREATE_DB_FILE_NAME);
		let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
		pexec(&root_db, &sql_recreate_db_file).await?;
	}

	let mut paths: Vec<PathBuf> = fs::read_dir(sql_dir)?
		.filter_map(|entry| entry.ok().map(|e| e.path()))
		.collect();
	paths.sort();

	let app_db = new_db_pool(PG_DEV_APP_URL).await?;
	for path in paths {
		let path_str = path.to_string_lossy();
		if path_str.ends_with(".sql")
			&& !path_str.ends_with(SQL_RECREATE_DB_FILE_NAME)
		{
			pexec(&app_db, &path).await?;
		}
	}

	// -- Init model layer
	let mm = ModelManager::new().await?;
	let ctx = Ctx::root_ctx();

	// -- Set demo1 pwd
	let demo1_user: User = UserBmc::first_by_username(&ctx, &mm, "demo1")
		.await?
		.unwrap();
	UserBmc::update_pwd(&ctx, &mm, demo1_user.id, DEMO_PWD).await?;
	info!("{:<12} - init_dev_db - set demo1 pwd", "FOR-DEV-ONLY");

	Ok(())
}

async fn pexec(db: &Db, file: &Path) -> Result<(), sqlx::Error> {
	info!("{:<12} - pexec: {file:?}", "FOR-DEV-ONLY");

	let content = fs::read_to_string(file)?;
	let sqls: Vec<&str> = content.split(';').collect();
	for sql in sqls {
		sqlx::query(sql).execute(db).await?;
	}
	Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(5)
		.acquire_timeout(Duration::from_millis(500))
		.connect(db_con_url)
		.await
}
