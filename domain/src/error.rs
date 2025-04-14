use atom_infrastructure::models::PostgrestResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("INFRASTRUCTURE_ERROR: {0}")]
    InfrastructureError(#[from] atom_infrastructure::error::InfrastructureError),
    #[error("SERDE_JSON_ERROR: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("UUID_ERROR: {0}")]
    Uuid(#[from] uuid::Error),
    #[error("REQWEST_ERROR: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("POSTGREST_ERROR: {0:#?}")]
    PostgrestError(PostgrestResponse),
}
