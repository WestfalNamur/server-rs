use crate::error;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(payload: Json<LoginPayload>) -> error::Result<Json<Value>> {
    println!("->> {:12} - api_login", "HANDLER");

    if payload.username != "usr1" || payload.password != "pwd1" {
        return Err(error::Error::LoginFail);
    }

    let body = Json(json!({
      "result": {
        "success": true
      }
    }));

    Ok(body)
}
