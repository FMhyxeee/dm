use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("delete error, no such id: {0}")]
    DeleteError(i32),

    #[error("update error, no such id: {0}")]
    UpdateError(i32),

    #[error("insert error, id already exists: {0}")]
    InsertError(i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let status_code = match self {
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DeleteError(_) => StatusCode::NOT_FOUND,
            AppError::UpdateError(_) => StatusCode::NOT_FOUND,
            AppError::InsertError(_) => StatusCode::CONFLICT,
        };

        (status_code, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
