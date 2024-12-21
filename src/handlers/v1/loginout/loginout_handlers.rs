use axum::{extract::State, Json};
use tower_cookies::cookie::time::Duration;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use crate::error::Error;
use crate::models::v1::loginout::login_payload::{LoginPayload, LoginResponse};
use crate::share_state::HandlerState;

pub const AUTH_TOKEN: &str = "auth-token";

pub struct LoginoutHandlers {
}

impl LoginoutHandlers {
    pub async fn api_login(cookies: Cookies, State(_handler_state): State<HandlerState>, payload: Json<LoginPayload>) -> Result<Json<LoginResponse>, Error>{
        // TODO: authenciate with username/password in DB later
        if payload.0.username != "test1" || payload.0.pwd != "hello" {
            return Err(Error::LoginFailed);
        }

        //TODO: add actual auth token
        let mut c = Cookie::new(AUTH_TOKEN, "user-1.exp.sign");
        c.set_max_age(Duration::hours(1));
        c.set_same_site(SameSite::Strict);
        c.set_secure(true);
        c.set_http_only(true);
        c.set_path("/");
        c.set_domain(".app.localhost");
        cookies.add(c);

        Ok(Json(LoginResponse {
            success: true,
            error: None
        }))
    }
}
