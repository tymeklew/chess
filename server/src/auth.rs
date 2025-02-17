use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode, Json};
use axum::{Extension, RequestPartsExt};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use bcrypt::{hash, DEFAULT_COST};
use futures::lock::Mutex;
use serde::Deserialize;
use sqlx::query;
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::error::AppError;
use crate::AppState;

const CREATE_USER: &str = r#"
    INSERT INTO users (user_id , email , username , password)
    VALUES ($1 , $2 , $3 , $4)
"#;
const USER_EXISTS: &str = r#"
SELECT
    CASE
        WHEN EXISTS (SELECT 1 FROM users WHERE email = $1) THEN 'email'
        WHEN EXISTS (SELECT 1 FROM users WHERE username = $2) THEN 'username'
        ELSE 'none'
    END AS conflict_field;
"#;

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Invalid username must be between 3 and 20 characters"
    ))]
    pub username: String,
    #[validate(length(min = 3, max = 72))]
    pub password: String,
}

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, AppError> {
    payload.validate()?;
    let pool = &state.pool;

    let result = query(USER_EXISTS)
        .bind(&payload.email)
        .bind(&payload.username)
        .fetch_one(pool)
        .await?;

    match result.get(0) {
        Some("email") => {
            return Err(AppError::ConflictError(
                "The provided email is already taken".into(),
            ))
        }
        Some("username") => {
            return Err(AppError::ConflictError(
                "The provided username is already taken".into(),
            ))
        }
        _ => {}
    };

    let id = Uuid::new_v4();
    let hashed_password = hash(payload.password, DEFAULT_COST)?;

    query(CREATE_USER)
        .bind(id)
        .bind(payload.email)
        .bind(payload.username)
        .bind(hashed_password)
        .execute(pool)
        .await?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub identifier: String,
    pub password: String,
}

const LOGIN_USER: &str = r#"
    SELECT user_id , password FROM users WHERE email = $1 OR username = $1
"#;
const CREATE_SESSION: &str = r#"
    INSERT INTO sessions (session_id , user_id)
    VALUES ($1 , $2)
"#;

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(payload): Json<LoginUser>,
) -> Result<CookieJar, AppError> {
    let pool = &state.pool;

    let user = query(LOGIN_USER)
        .bind(&payload.identifier)
        .fetch_optional(pool)
        .await?;
    if user.is_none() {
        return Err(StatusCode::UNAUTHORIZED.into());
    }
    let row = user.unwrap();
    let user_id: Uuid = row.get(0);
    let hashed_password: String = row.get(1);

    if bcrypt::verify(payload.password, &hashed_password)? == false {
        return Err(StatusCode::UNAUTHORIZED.into());
    };

    let session_id = Uuid::new_v4();
    query(CREATE_SESSION)
        .bind(session_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(jar.add(Cookie::new("session_id", session_id.to_string())))
}

pub struct AuthenticatedUser(pub Uuid);

const SESSION_QUERY: &str = r#"
    SELECT user_id FROM sessions
    WHERE session_id = $1 AND created_at > CURRENT_TIMESTAMP - INTERVAL '2 weeks' 
"#;
#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let session_id = match jar.get("session_id") {
            Some(id) => Uuid::parse_str(id.value())
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?,
            None => return Err(StatusCode::UNAUTHORIZED.into_response()),
        };

        let Extension(state) = parts
            .extract::<Extension<Arc<AppState>>>()
            .await
            .map_err(|err| err.into_response())?;
        let pool = &state.pool;

        log::debug!("Session id : {}", session_id);
        let res = query(SESSION_QUERY)
            .bind(session_id)
            .fetch_optional(pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;
        log::debug!("After request");

        let row = match res {
            Some(row) => row,
            None => return Err(StatusCode::UNAUTHORIZED.into_response()),
        };
        let user_id: Uuid = row.get(0);

        Ok(AuthenticatedUser(user_id))
    }
}
