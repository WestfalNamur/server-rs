use crate::error::{Error, Result};
use crate::web::AUTH_TOKEN;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

pub async fn mw_require_auth<Body>(
    cookies: Cookies,
    req: Request<Body>,
    next: Next<Body>,
) -> Result<Response> {
    // TODO: Real auth
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    auth_token.ok_or(Error::AuthFailNoTokenCookie)?; // "?" to return early;

    Ok(next.run(req).await)
}
