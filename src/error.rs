use http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    ConfigMissingEnv(&'static str),
    LoginFailed
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::LoginFailed => {
                (StatusCode::UNAUTHORIZED, "LOGIN_FAILED").into_response()
            },
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, "GENERIC_ERROR").into_response()
            }
        }
    }
}