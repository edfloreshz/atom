use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("DATABASE_ERROR")]
    DatabaseError,
    #[error("DOTENV_ERROR: {0}")]
    DotEnvError(#[from] dotenv::Error),
}
