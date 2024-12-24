use axum::{extract::State, Json};
use rand::distributions::Alphanumeric;
use rand::Rng;
use tower_cookies::cookie::time::Duration;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use crate::error::Error;
use crate::models::v1::loginout::login_payload::{LoginPayload, LoginResponse};
use crate::share_state::HandlerState;

pub const SESSION_ID: &str = "id";
pub const SESSION_KEY_LEN: usize = 64;

pub struct LoginoutHandlers {
}

impl LoginoutHandlers {
    pub async fn api_login(cookies: Cookies, State(_handler_state): State<HandlerState>, payload: Json<LoginPayload>) -> Result<Json<LoginResponse>, Error>{
        // TODO: authenciate with username/password in DB later
        if payload.0.username != "test1" || payload.0.pwd != "hello" {
            return Err(Error::LoginFailed);
        }

        //TODO: add actual auth 

        // create session key
        let mut c = Cookie::new(SESSION_ID, create_session_key().await);
        c.set_max_age(Duration::hours(1));
        c.set_same_site(SameSite::Strict);
        c.set_secure(true);
        c.set_http_only(true);
        c.set_path("/");
        c.set_domain(".app.localhost");
        cookies.add(c);

        // TODO: save session into redis

        Ok(Json(LoginResponse {
            success: true,
            error: None
        }))
    }
}

async fn create_session_key() -> String {
    let session_key = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SESSION_KEY_LEN)
        .map(char::from)
        .collect();
    session_key
}
