use crate::error::DomainError;

#[allow(async_fn_in_trait)]
pub trait Crud<K, V> {
    /// Read all values.
    async fn read_all(&self) -> Result<Vec<V>, DomainError>;
    /// Read the value based on key.
    async fn read(&self, id: &K) -> Result<Option<V>, DomainError>;
    /// Create a new value based on key and value.
    async fn create(&self, value: &V) -> Result<V, DomainError>;
    /// Update the value based on key and return it.
    async fn update(&self, id: K, value: V) -> Result<V, DomainError>;
    /// Delete the value based on key and return it.
    async fn delete(&self, id: &K) -> Result<Option<V>, DomainError>;
    /// Check if the value exists based on key.
    async fn exists(&self, id: &K) -> Result<bool, DomainError> {
        Ok(self.read(id).await?.is_some())
    }
}
