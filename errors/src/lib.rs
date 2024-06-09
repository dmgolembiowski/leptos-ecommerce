use http::status::StatusCode;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use strum_macros::EnumString;
#[derive(Debug, Clone, EnumString, Error, Diagnostic, Serialize, Deserialize)]
pub enum EcommerceAppError {
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

impl EcommerceAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            EcommerceAppError::NotFound => StatusCode::NOT_FOUND,
            EcommerceAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            EcommerceAppError::InvalidDateTime => StatusCode::BAD_REQUEST,
            EcommerceAppError::MissingOrInvalidFrontmatter => StatusCode::INTERNAL_SERVER_ERROR,
            EcommerceAppError::TomlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}