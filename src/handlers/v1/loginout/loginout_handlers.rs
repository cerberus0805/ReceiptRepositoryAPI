use axum::{extract::State, Json};
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
        cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

        Ok(Json(LoginResponse {
            success: true,
            error: None
        }))
    }
}
