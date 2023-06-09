use crate::{error, web};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> error::Result<Json<Value>> {
    println!("->> {:12} - api_login", "HANDLER");

    if payload.username != "usr1" || payload.password != "pwd1" {
        return Err(error::Error::LoginFail);
    }

    // TODO: Implement real auth-token
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
      "result": {
        "success": true
      }
    }));

    Ok(body)
}
