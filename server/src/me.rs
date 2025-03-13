use std::sync::Arc;
use serde::Serialize;
use axum::{extract::State, Json};
use uuid::Uuid;

use crate::{auth::AuthenticatedUser, error::AppError, AppState};

#[derive(Serialize , sqlx::FromRow)]
pub struct Me {
    username : String,
    email : String,
}

const ME_QUERY : &str = r#"
SELECT username , email
FROM users
WHERE user_id = $1
"#;
pub async fn me(
    State(state) : State<Arc<AppState>>,
    user :AuthenticatedUser 
) -> Result<Json<Me> , AppError> {
    let res : Me = sqlx::query_as(ME_QUERY).bind(user.0).fetch_one(&state.pool).await?;
    Ok(Json(res))
} 

// Fetch all bot games and player games and organise them in a single endpoint
const PLAYER_GAMES_QUERY : &str = r#"
SELECT users.user_id , users.username , player_games.started , player_games.finished , player_games.winner ,    CASE
        WHEN player_games.white_id = $1 THEN true
        ELSE false
    END AS white
FROM player_games
INNER JOIN users ON users.user_id =
                    CASE
                        WHEN player_games.black_id = $1 THEN player_games.white_id
                        ELSE player_games.black_id
                    END
WHERE player_games.white_id = $1 OR player_games.black_id = $1;
"#;
#[derive(Serialize , sqlx::FromRow)]
pub struct PlayerGame {
    user_id : Uuid,
    username : String,
    started : chrono::DateTime<chrono::Utc>,
    finished : Option<chrono::DateTime<chrono::Utc>>,
    winner : Option<Uuid>, 
    white : bool
}
pub async fn player_games(
    State(state) : State<Arc<AppState>>,
    user : AuthenticatedUser
) -> Result<Json<Vec<PlayerGame>> , AppError> {
    let res : Vec<PlayerGame> = sqlx::query_as(PLAYER_GAMES_QUERY).bind(user.0).fetch_all(&state.pool).await?;
    Ok(Json(res))
}

#[derive(Serialize , sqlx::FromRow)]
pub struct BotGame {
    pub id : Uuid,
    pub difficulty : i32,
    pub white : bool,
    pub won : bool
}
const BOT_GAMES_QUERY : &str = r#"
SELECT id, difficulty , white , won
FROM bot_games
WHERE player_id = $1
"#;
pub async fn bot_games(
    State(state) : State<Arc<AppState>>,
    user : AuthenticatedUser
) -> Result<Json<Vec<BotGame>> , AppError> {
    log::info!("User : {}", user.0);
    let res : Vec<BotGame> = sqlx::query_as(BOT_GAMES_QUERY).bind(user.0).fetch_all(&state.pool).await?;
    Ok(Json(res))
}