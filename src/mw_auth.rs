use crate::{error::Error, handlers::v1::loginout::loginout_handlers::AUTH_TOKEN};
use axum::{{body::Body, http::Request}, middleware::Next};
use tower_cookies::Cookies;
use axum::response::Response;
use tracing::info;

pub async fn mw_require_auth(
    cookies: Cookies, 
    req: Request<Body>, 
    next: Next
) -> Result<Response, Error> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    info!("MIDDLEAWARE: {:#?}", auth_token);
    //TODO: auth-token parsing and validation
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    
    Ok(next.run(req).await)
}
  