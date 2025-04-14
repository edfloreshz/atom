use std::sync::Arc;

use atom_domain::{crud::Crud, services::habit::HabitService};
use atom_infrastructure::Postgrest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ApiError, prelude::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct Habit {
    id: Uuid,
}

#[derive(Clone)]
pub struct HabitState {
    pub service: HabitService,
}

impl From<atom_infrastructure::models::Habit> for Habit {
    fn from(habit: atom_infrastructure::models::Habit) -> Self {
        Habit { id: habit.id }
    }
}

impl From<Habit> for atom_infrastructure::models::Habit {
    fn from(habit: Habit) -> Self {
        atom_infrastructure::models::Habit { id: habit.id }
    }
}

impl Habit {
    pub fn routes(client: Arc<Postgrest>) -> axum::Router {
        axum::Router::new()
            .route(
                "/api/habit/{id}",
                get(Habit::get).put(Habit::update).delete(Habit::delete),
            )
            .route("/api/habit", get(Habit::all).post(Habit::create))
            .with_state(HabitState {
                service: HabitService::new(client),
            })
    }

    pub async fn all(
        State(HabitState { service }): State<HabitState>,
    ) -> Result<Json<Vec<Habit>>, ApiError> {
        let habits = service
            .read_all()
            .await?
            .into_iter()
            .map(|item| item.into())
            .collect();
        Ok(Json(habits))
    }

    pub async fn get(
        State(HabitState { service }): State<HabitState>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Option<Habit>>, ApiError> {
        let habit = service.read(&id).await?.map(|item| item.into());
        Ok(Json(habit))
    }

    pub async fn create(
        State(HabitState { service }): State<HabitState>,
        Json(habit): Json<Habit>,
    ) -> Result<Json<Habit>, ApiError> {
        let habit = service.create(&habit.into()).await?;
        Ok(Json(habit.into()))
    }

    pub async fn update(
        State(HabitState { service }): State<HabitState>,
        Path(id): Path<Uuid>,
        Json(habit): Json<Habit>,
    ) -> Result<Json<Habit>, ApiError> {
        let habit = service.update(id, habit.into()).await?;
        Ok(Json(habit.into()))
    }

    pub async fn delete(
        State(HabitState { service }): State<HabitState>,
        Path(id): Path<Uuid>,
    ) -> Result<(), ApiError> {
        service.delete(&id).await?;
        Ok(())
    }
}
