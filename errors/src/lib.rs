use http::status::StatusCode;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Clone, Error, Diagnostic, Serialize, Deserialize)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("TomlError: {0}")]
    TomlError(String),
    #[error("Invalid Date or Time")]
    InvalidDateTime,
    #[error("Missing or Invalid Frontmatter")]
    MissingOrInvalidFrontmatter,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidDateTime => StatusCode::BAD_REQUEST,
            AppError::MissingOrInvalidFrontmatter => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TomlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}