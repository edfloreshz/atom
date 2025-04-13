use crate::database::error::DatabaseError;

pub enum InfrastructureError {
    DatabaseError(DatabaseError),
}

impl From<DatabaseError> for InfrastructureError {
    fn from(error: DatabaseError) -> Self {
        InfrastructureError::DatabaseError(error)
    }
}
