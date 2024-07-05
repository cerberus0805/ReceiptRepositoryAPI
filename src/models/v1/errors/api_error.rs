use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error, Serialize)]
pub enum ApiError {
    #[error("Generic error")]
    Generic,
    #[error("Record not found")]
    NoRecord(String),
    #[error("Database disconnect")]
    DatabaseConnectionBroken
}