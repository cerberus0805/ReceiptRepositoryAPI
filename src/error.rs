use http::StatusCode;
use axum::{response::{IntoResponse, Response}, Json};

use crate::models::v1::loginout::login_payload::LoginResponse;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    ConfigMissingEnv(&'static str),
    LoginFailed,
    AuthFailNoAuthTokenCookie
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::LoginFailed => {
                (StatusCode::UNAUTHORIZED, Json(LoginResponse{ success: false, error: Some("LoginFailed".to_string())})).into_response()
            },
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(LoginResponse{ success: false, error: Some("GenericFailed".to_string())})).into_response()
            }
        }
    }
}