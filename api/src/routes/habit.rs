use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ApiError, prelude::*, route::Route};

#[derive(Debug, Serialize, Deserialize)]
pub struct Habit {
    id: Uuid,
}

impl Route for Habit {
    fn routes() -> axum::Router {
        axum::Router::new()
            .route(
                "/api/habit/{id}",
                get(Habit::get).put(Habit::update).delete(Habit::delete),
            )
            .route("/api/habit", get(Habit::all).post(Habit::create))
    }
}

impl Habit {
    pub async fn all() -> Result<Json<Vec<Habit>>, ApiError> {
        Ok(Json(vec![Habit { id: Uuid::new_v4() }]))
    }

    pub async fn get(Path(id): Path<Uuid>) -> Result<Json<Habit>, ApiError> {
        Ok(Json(Habit { id }))
    }

    pub async fn create(Json(habit): Json<Habit>) -> Result<Json<Habit>, ApiError> {
        Ok(Json(habit))
    }

    pub async fn update(
        Path(id): Path<Uuid>,
        Json(mut habit): Json<Habit>,
    ) -> Result<Json<Habit>, ApiError> {
        habit.id = id;
        Ok(Json(habit))
    }

    pub async fn delete(Path(id): Path<Uuid>) -> Result<Json<Habit>, ApiError> {
        Ok(Json(Habit { id }))
    }
}
