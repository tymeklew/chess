use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::{error::ErrorKind, query};
use std::sync::Arc;
use uuid::Uuid;

use crate::{auth::AuthenticatedUser, error::AppError, AppState};

#[derive(Deserialize)]
pub struct FriendRequest {
    friend_id: Uuid,
}

const FRIEND_REQUEST: &str = r#"
    INSERT INTO friend_requests (request_id , user_id , friend_id)
    VALUES ($1 , $2 , $3)
"#;
pub async fn friend_request(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Json(payload): Json<FriendRequest>,
) -> Result<StatusCode, AppError> {
    let request_id = Uuid::new_v4();

    let pool = &state.pool;

    match query(FRIEND_REQUEST)
        .bind(request_id)
        .bind(user.0)
        .bind(payload.friend_id)
        .execute(pool)
        .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => match e.as_database_error() {
            Some(err) if err.kind() == ErrorKind::UniqueViolation => {
                Err(StatusCode::CONFLICT.into())
            }
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR.into()),
        },
    }
}
