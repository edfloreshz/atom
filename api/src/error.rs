use atom_domain::error::DomainError;
use atom_infrastructure::error::InfrastructureError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("NOT_FOUND: {0}")]
    NotFound(String),
    #[error("ALREADY_EXISTS: {0}")]
    AlreadyExists(String),
    #[error("INVALID_PARAMS")]
    InvalidParams,
    #[error("SERVICE_ERROR")]
    ServiceError(Option<String>),
    #[error("DOMAIN_ERROR: {0}")]
    DomainError(#[from] DomainError),
    #[error("INFRASTRUCTURE_ERROR: {0}")]
    InfrastructureError(#[from] InfrastructureError),
    #[error("AXUM_ERROR: {0}")]
    AxumError(#[from] axum::Error),
    #[error("IO_ERROR: {0}")]
    Io(#[from] std::io::Error),
}

impl ApiError {
    pub fn description(&self) -> Option<String> {
        match self {
            Self::NotFound(e) => Some(e.clone()),
            Self::InvalidParams => Some("Invalid parameters".to_string()),
            Self::ServiceError(e) => e.clone(),
            Self::AlreadyExists(e) => Some(e.clone()),
            Self::DomainError(domain_error) => Some(domain_error.to_string()),
            Self::InfrastructureError(infrastructure_error) => {
                Some(infrastructure_error.to_string())
            }
            Self::AxumError(axum_error) => Some(axum_error.to_string()),
            Self::Io(io_error) => Some(io_error.to_string()),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidParams => StatusCode::BAD_REQUEST,
            Self::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            Self::DomainError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InfrastructureError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AxumError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let Some(description) = self.description() else {
            return self.status_code().into_response();
        };

        (self.status_code(), description).into_response()
    }
}
