use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
use lib_auth::pwd::{self, ContentToHash};
use modql::field::{Fields, HasFields};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow, Fields)]
pub struct User {
	pub id: i64,
	pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
	pub username: String,
	pub pwd_clear: String,
}

#[derive(Fields)]
pub struct UserForInsert {
	pub username: String,
}

#[derive(Fields, FromRow, Clone, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub username: String,

	pub pwd: Option<String>,
	pub pwd_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, Debug, FromRow, Fields)]
pub struct UserForAuth {
	pub id: i64,
	pub username: String,
	pub token_salt: Uuid,
}

pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

#[derive(Iden)]
enum UserIden {
	Id,
	Username,
	Pwd,
}

pub struct UserBmc;
impl DbBmc for UserBmc {
	const TABLE: &'static str = "user";
}

impl UserBmc {
	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_username<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		username: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		let db = mm.db();
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::field_idens())
			.and_where(Expr::col(UserIden::Username).eq(username));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let user = sqlx::query_as_with::<_, E, _>(&sql, values)
			.fetch_optional(db)
			.await?;
		Ok(user)
	}

	pub async fn update_pwd(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		pwd_clear: &str,
	) -> Result<()> {
		let db = mm.db();

		let user: UserForLogin = Self::get(ctx, mm, id).await?;
		let pwd = pwd::hash_pwd(&ContentToHash {
			content: pwd_clear.to_string(),
			salt: user.pwd_salt,
		})?;

		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.value(UserIden::Pwd, SimpleExpr::from(pwd))
			.and_where(Expr::col(UserIden::Id).eq(id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let _count = sqlx::query_with(&sql, values)
			.execute(db)
			.await?
			.rows_affected();
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_dev_utils;
	use anyhow::{Context, Result};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_first_ok_demo1() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "demo1";

		let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
			.await?
			.context("Should have user 'demo1'")?;
		assert_eq!(user.username, fx_username);
		Ok(())
	}
}
