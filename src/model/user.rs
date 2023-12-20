use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};

use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
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
}
