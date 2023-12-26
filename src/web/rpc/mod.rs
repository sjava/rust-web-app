mod task_rpc;

use std::sync::Arc;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::rpc::task_rpc::{create_task, delete_task, list_tasks, update_task};
use crate::web::{Error, Result};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;

#[derive(Deserialize)]
struct RpcRequest {
	id: Option<Value>,
	method: String,
	params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
	id: i64,
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
	id: i64,
}

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/rpc", post(rpc_handler))
		.with_state(mm)
}

async fn rpc_handler(
	State(mm): State<ModelManager>,
	ctx: Ctx,
	Json(rpc_req): Json<RpcRequest>,
) -> Response {
	let rpc_info = RpcInfo {
		id: rpc_req.id.clone(),
		method: rpc_req.method.clone(),
	};

	let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
	res.extensions_mut().insert(Arc::new(rpc_info));

	res
}

#[derive(Debug)]
pub struct RpcInfo {
	pub id: Option<Value>,
	pub method: String,
}

macro_rules! exec_rpc_fn {
	($rpc_fn:expr,$ctx:expr,$mm:expr,$rpc_params:expr) => {{
		let rpc_fn_name = stringify!($rpc_fn);
		let params = $rpc_params.ok_or(Error::RpcMissingParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;

		let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;
		$rpc_fn($ctx, $mm, params).await.map(to_value)??
	}};

	($rpc_fn:expr,$ctx:expr,$mm:expr) => {
		$rpc_fn($ctx, $mm).await.map(to_value)??
	};
}

async fn _rpc_handler(
	ctx: Ctx,
	mm: ModelManager,
	rpc_req: RpcRequest,
) -> Result<Json<Value>> {
	let RpcRequest {
		id: rpc_id,
		method: rpc_method,
		params: rpc_params,
	} = rpc_req;

	debug!("{:<12} - _rpc_handler: method: {rpc_method}", "HANDLER");

	let result_json: Value = match rpc_method.as_str() {
		"create_task" => exec_rpc_fn!(create_task, ctx, mm, rpc_params),
		"list_tasks" => exec_rpc_fn!(list_tasks, ctx, mm, rpc_params),
		"update_task" => exec_rpc_fn!(update_task, ctx, mm, rpc_params),
		"delete_task" => exec_rpc_fn!(delete_task, ctx, mm, rpc_params),
		_ => return Err(Error::RpcMethodUnknow(rpc_method)),
	};

	let body_response = json!({
		"id": rpc_id,
		"result": result_json
	});

	Ok(Json(body_response))
}
