use std::sync::Arc;

use atom_infrastructure::{models::Habit, Postgrest};
use uuid::Uuid;

use crate::{crud::Crud, error::DomainError};

#[derive(Clone)]
pub struct HabitService {
    database: Arc<Postgrest>,
    table: String,
}

impl HabitService {
    pub fn new(database: Arc<Postgrest>) -> Self {
        Self {
            database,
            table: "habit".into(),
        }
    }
}

impl Crud<Uuid, Habit> for HabitService {
    async fn read_all(&self) -> Result<Vec<Habit>, crate::error::DomainError> {
        let response = self
            .database
            .from(&self.table)
            .select("*")
            .execute()
            .await?
            .text()
            .await?;

        let habits = serde_json::from_str(&response)?;
        Ok(habits)
    }

    async fn read(&self, id: &Uuid) -> Result<Option<Habit>, crate::error::DomainError> {
        let response = self
            .database
            .from(&self.table)
            .eq("id", id.to_string())
            .single()
            .execute()
            .await?;

        let body = response.text().await?;

        let Ok(habit) = serde_json::from_str(&body) else {
            return Err(DomainError::PostgrestError(serde_json::from_str(&body)?));
        };

        Ok(Some(habit))
    }

    async fn create(&self, value: &Habit) -> Result<Habit, crate::error::DomainError> {
        let response = self
            .database
            .from(&self.table)
            .insert(serde_json::to_string(value)?)
            .select("*")
            .execute()
            .await?
            .text()
            .await?;

        let habit = serde_json::from_str(&response)?;

        Ok(habit)
    }

    async fn update(&self, id: Uuid, value: Habit) -> Result<Habit, crate::error::DomainError> {
        let response = self
            .database
            .from(&self.table)
            .update(serde_json::to_string(&value)?)
            .eq("id", id.to_string())
            .select("*")
            .execute()
            .await?
            .text()
            .await?;

        let habit = serde_json::from_str(&response)?;

        Ok(habit)
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<Habit>, crate::error::DomainError> {
        let response = self
            .database
            .from(&self.table)
            .delete()
            .eq("id", id.to_string())
            .select("*")
            .execute()
            .await?
            .text()
            .await?;

        let habit = serde_json::from_str(&response)?;

        Ok(Some(habit))
    }
}
