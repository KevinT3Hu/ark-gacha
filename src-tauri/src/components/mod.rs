use std::sync::mpsc::SendError;

use serde::Serialize;

pub mod auth;
pub mod gacha;
pub(in crate::components) mod internal_db;
pub mod statistics;

#[derive(thiserror::Error, Debug)]
pub enum HandlerExecutionError {
    #[error("Failed to perform IO operation: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Failed to perform json (de)serialization: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Failed to perform reqwest operation: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("{0}")]
    UserError(String),

    #[error("Failed to perform database operation: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("Failed to send data to database thread: {0}")]
    SendError(#[from] SendError<Vec<internal_db::Gacha>>),
}

impl Serialize for HandlerExecutionError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type HandlerResult<T> = Result<T, HandlerExecutionError>;
