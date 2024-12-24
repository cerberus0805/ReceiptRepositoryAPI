use crate::{error::Error, handlers::v1::loginout::loginout_handlers::SESSION_ID};
use axum::{{body::Body, http::Request}, middleware::Next};
use tower_cookies::Cookies;
use axum::response::Response;
use tracing::info;

pub async fn mw_require_auth(
    cookies: Cookies, 
    req: Request<Body>, 
    next: Next
) -> Result<Response, Error> {
    let session_id = cookies.get(SESSION_ID).map(|c| c.value().to_string());
    info!("MIDDLEAWARE: {:#?}", session_id);
    //TODO: session_id and validation
    session_id.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    
    Ok(next.run(req).await)
}
  