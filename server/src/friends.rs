use crate::{auth::AuthenticatedUser, error::AppError, AppState};
use axum::extract::Query;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::{error::ErrorKind, query};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SearchUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    user_id: Uuid,
    username: String,
}

const SEARCH_QUERY: &str = r#"
SELECT user_id , username 
FROM users
WHERE SIMILARITY(username , $1) > 0.3
ORDER BY SIMILARITY(username , $1)
LIMIT 10;
"#;
pub async fn search_user(
    State(state): State<Arc<AppState>>,
    _: AuthenticatedUser,
    search: Query<SearchUser>,
) -> Result<Json<Vec<User>>, AppError> {
    let res = query(SEARCH_QUERY)
        .bind(search.username.clone())
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(
        res.iter()
            .map(|row| User {
                user_id: row.get(0),
                username: row.get(1),
            })
            .collect(),
    ))
}

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

#[derive(Deserialize)]
pub struct FriendRequestCancellation {
    request_id: Uuid,
}

const CANCEL_FRIEND_REQUEST: &str = r#"
    UPDATE friend_requests
    WHERE request_id = $1
    SET status = 'cancelled'
"#;
pub async fn cancel_friend_request(
    State(state): State<Arc<AppState>>,
    _: AuthenticatedUser,
    Json(payload): Json<FriendRequestCancellation>,
) -> Result<StatusCode, AppError> {
    query(CANCEL_FRIEND_REQUEST)
        .bind(payload.request_id)
        .execute(&state.pool)
        .await?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct FriendRequestResponse {
    request_id: Uuid,
    response: bool,
}

const REJECT_FRIEND_REQUEST: &str = r#"
    UPDATE friend_requests
    WHERE request_id = $1
    SET status = 'rejected'
"#;
const ACCEPT_FRIEND_REQUEST: &str = r#"
    UPDATE friend_requests
    WHERE request_id = $1
    SET status = 'accepted'
"#;
const GET_USER_ID: &str = r#"
    SELECT user_id 
    FROM friend_requests
    WHERE request_id = $1
"#;
const CREATE_FRIENDSHIP: &str = r#"
    INSERT INTO friendships (user_id , friend_id)
    VALUES ($1 , $2)
"#;
pub async fn respond_to_friend_request(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Json(payload): Json<FriendRequestResponse>,
) -> Result<StatusCode, AppError> {
    match payload.response {
        true => {
            let res = query(GET_USER_ID)
                .bind(payload.request_id)
                .fetch_optional(&state.pool)
                .await?;
            if res.is_none() {
                return Err(StatusCode::NOT_FOUND.into());
            }
            let res = res.unwrap();
            let user_id: Uuid = res.get(0);

            query(ACCEPT_FRIEND_REQUEST)
                .bind(payload.request_id)
                .bind(user.0)
                .execute(&state.pool)
                .await?;
            query(CREATE_FRIENDSHIP)
                .bind(user_id)
                .bind(user.0)
                .execute(&state.pool)
                .await?;
        }
        false => {
            query(REJECT_FRIEND_REQUEST)
                .bind(payload.request_id)
                .bind(user.0)
                .execute(&state.pool)
                .await?;
        }
    }

    Ok(StatusCode::CREATED)
}
