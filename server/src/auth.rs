use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, DEFAULT_COST};
use futures::lock::Mutex;
use sqlx::{pool, query};
use validator::{Validate, ValidationError};
use uuid::Uuid;

use crate::{AppError, AppState};

const CREATE_USER: &str = r#"
    INSERT INTO users (user_id , email , username , password)
    VALUES ($1 , $2 , $3 , $4)
"#;

#[derive(serde::Deserialize , Validate)]
pub struct CreateUser {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 3 , max = 20 , message = "Invalid username must be between 3 and 20 characters"))]
    pub username: String,
    #[validate(length(min = 3 , max = 72))]
    pub password: String,
}


pub async fn signup(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, AppError> {
    payload.validate()?;

    let id = Uuid::new_v4();
    let hashed_password = hash(payload.password, DEFAULT_COST)?;

    let pool = &state.lock().await.pool;

    query(CREATE_USER)
        .bind(id)
        .bind(payload.email)
        .bind(payload.username)
        .bind(hashed_password)
        .execute(pool)
        .await?;

    Ok(StatusCode::CREATED)
}

pub async fn login(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    todo!()
}
