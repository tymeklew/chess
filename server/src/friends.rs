use crate::{auth::AuthenticatedUser, error::AppError, AppState};
use anyhow::Result;
use axum::extract::Query;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Row};
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
pub struct NewFriendRequest {
    friend_id: Uuid,
}

const FRIEND_REQUEST: &str = r#"
    INSERT INTO friend_requests (request_id , user_id , friend_id)
    VALUES ($1 , $2 , $3)
"#;
pub async fn friend_request(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Json(payload): Json<NewFriendRequest>,
) -> Result<StatusCode, AppError> {
    let request_id = Uuid::new_v4();

    let pool = &state.pool;
    log::info!("{} , {} , {}" , request_id , user.0 , payload.friend_id);

    match query(FRIEND_REQUEST)
        .bind(request_id)
        .bind(user.0)
        .bind(payload.friend_id)
        .execute(pool)
        .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => match e.as_database_error() {
            Some(err) if err.kind() == ErrorKind::UniqueViolation || err.kind() == ErrorKind::ForeignKeyViolation => {
                Err(StatusCode::CONFLICT.into())
            }
            _ => {
                Err(StatusCode::INTERNAL_SERVER_ERROR.into())},
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

const FRIENDS_QUERY : &str = r#"
SELECT users.username , users.user_id
FROM friendships f
JOIN users ON users.user_id =
              CASE
                  WHEN f.user_id = $1 THEN f.friend_id
                  ELSE f.user_id
              END
WHERE $1 IN (f.user_id , f.friend_id)
"#;
pub async fn list_friends(State(state) : State<Arc<AppState>> , user : AuthenticatedUser) -> Result<Json<Vec<User>> , AppError> {
    let res = query(FRIENDS_QUERY)
        .bind(user.0)
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

#[derive(Deserialize , Serialize , sqlx::FromRow , Debug)]
pub struct FriendRequest {
    request_id : Uuid,
    // Username of either the user or the friend
    username : String,
    //Who sent the friend request
    incoming : bool,
    status : String,
}
const FRIEND_REQUESTS_QUERY : &str = r#"
SELECT users.username, subquery.incoming , subquery.status::text As status , subquery.request_id
FROM (
    SELECT req.*,
           CASE
               WHEN friend_id = $1 THEN TRUE
               ELSE FALSE
           END AS incoming
    FROM friend_requests req
    WHERE req.friend_id = $1 
       OR req.user_id = $1
) AS subquery
JOIN users ON users.user_id =
    CASE
        WHEN subquery.incoming = TRUE THEN subquery.user_id
        ELSE subquery.friend_id
    END;
"#; 
#[axum::debug_handler]
pub async fn list_friend_requests(State(state) : State<Arc<AppState>> , user : AuthenticatedUser) -> Result<Json<Vec<FriendRequest>> , AppError> {
    let res : Vec<FriendRequest> = query_as(FRIEND_REQUESTS_QUERY)
        .bind(user.0)
        .fetch_all(&state.pool)
        .await?; 

    Ok(Json(res))
}
