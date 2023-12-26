#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	// hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	let mut task_ids: Vec<i64> = Vec::new();
	for i in 0..=4 {
		let req_create_task = hc.do_post(
			"/api/rpc",
			json!({
				"id": 1,
				"method": "create_task",
				"params": {
					"data": {"title": format!("task AAA {i}")}
				}
			}),
		);
		let resp = req_create_task.await?;
		task_ids.push(resp.json_value::<i64>("/result/id")?);
	}

	let req_update_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": task_ids[0],
			"method": "update_task",
			"params": {
			"data": {"id": 1000, "title": "task BB"}
			}
		}),
	);
	req_update_task.await?.print().await?;

	let req_delete_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_task",
			"params": {
			"data": {"id": task_ids[1]}
			}
		}),
	);
	req_delete_task.await?.print().await?;

	let req_list_tasks = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_tasks",
			"params": {
				"filters": [{
					"title": {"$endsWith": "BB"},
					"done": false,
				},{
					"id": {"$in": [task_ids[2], task_ids[3]]}
				}],
				"list_options": {
					"order_bys": "!id"
				}
			}
		}),
	);
	req_list_tasks.await?.print().await?;

	let req_logoff = hc.do_post("/api/req_logoff", json!({"logoff":true}));

	Ok(())
}
