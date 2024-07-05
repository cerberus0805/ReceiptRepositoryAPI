use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error, Serialize)]
pub enum ApiError {
    #[error("Generic error")]
    Generic,
    #[error("Invalid parameter")]
    InvalidParameter,
    #[error("Record not found")]
    NoRecord,
    #[error("Database disconnect")]
    DatabaseConnectionBroken
}