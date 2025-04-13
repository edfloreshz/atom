use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, Clone)]
pub enum ApiError {
    NotFound(String),
    AlreadyExists(String),
    InvalidParams,
    ServiceError(Option<String>),
}

impl ApiError {
    pub fn description(&self) -> Option<String> {
        match self {
            Self::NotFound(e) => Some(e.clone()),
            Self::InvalidParams => Some("Invalid parameters".to_string()),
            Self::ServiceError(e) => e.clone(),
            Self::AlreadyExists(e) => Some(e.clone()),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidParams => StatusCode::BAD_REQUEST,
            Self::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AlreadyExists(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(_) => write!(f, "NOT_FOUND",),
            Self::InvalidParams => write!(f, "INVALID_PARAMS"),
            Self::ServiceError(_) => write!(f, "SERVICE_ERROR"),
            Self::AlreadyExists(_) => write!(f, "ALREADY_EXISTS"),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let mut response = self.status_code().into_response();
        response.extensions_mut().insert(self);

        response
    }
}
