mod auth;
mod error;
mod friends;
mod game;
mod lobby;
mod player;
mod me;

use crate::game::{BotGame, Game};
use crate::lobby::Lobby;
use auth::AuthenticatedUser;
use axum::extract::ws::{WebSocket};
use axum::extract::{ConnectInfo, Query, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::{any, get, post};
use axum::{Extension, Router};
use game::PlayerGame;
use log::info;
use player::Player;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppState {
    pool: sqlx::PgPool,
    lobby: Mutex<Lobby<Player>>,
}

impl AppState {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            lobby: Mutex::new(Lobby::new()),
        }
    }

    pub async fn bot_game(&self, player: Player , difficulty : Option<String> , side : Option<String>) {
        if difficulty.is_none() || side.is_none() {
            info!("Insufficient data for bot game");
            return;
        }
        info!("Continue");
        let game = BotGame::new(player , self.pool.clone() , difficulty.unwrap() , side.unwrap());
        info!("Continue after");
        tokio::spawn(async move {
            game.start().await;
        });
    }
    pub async fn player_game(&self , player : Player) {
        let mut lobby = self.lobby.lock().unwrap();
        lobby.enqueue(player);
        info!("Enque : {}", lobby.size());

        if lobby.size() == 2 {
            info!("match found");
            let white = lobby.dequeue().unwrap();
            let black = lobby.dequeue().unwrap();

            let game = PlayerGame::new(white, black, self.pool.clone());
            tokio::spawn(
                async  move {
                    game.start().await;
                }
            );
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    // Setup the logger
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    //TODO New game implementation
    let state = Arc::new(AppState::new(pool));

    let app = Router::new()
        .route("/api/ws/play", any(ws_handler))
        .route("/api/me", get(me::me))
        .route("/api/games/bot" , get(me::bot_games))
        .route("/api/games/competitive" , get(me::player_games))
        .route("/api/friends/list", get(friends::list_friends))
        .route("/api/friends/request", post(friends::friend_request))
        .route("/api/friends/requests/list" , get(friends::list_friend_requests))
        .route(
            "/api/friends/response",
            post(friends::respond_to_friend_request),
        )
        .route("/api/friends/cancel", post(friends::cancel_friend_request))
        .route("/api/friends/search", get(friends::search_user))
        .layer(Extension(state.clone()))
        .route("/api/auth/signup", post(auth::signup))
        .route("/api/auth/login", post(auth::login))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    info!("Listening on : {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

#[derive(Deserialize)]
pub struct GameQuery {
    game: String,
    difficulty: Option<String>,
    side : Option<String>
}
async fn ws_handler(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(payload): Query<GameQuery>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    ws.on_upgrade(move |sock| handle_socket(sock, addr, state, user, payload))
}

async fn handle_socket(
    sock: WebSocket,
    addr: SocketAddr,
    state: Arc<AppState>,
    user: AuthenticatedUser,
    payload: GameQuery,
) {
    let player = Player::new(user.0 , sock);

    log::info!("Started : {}" , player.id);
    match payload.game.as_str() {
        "bot" => state.bot_game(player , payload.difficulty , payload.side).await,
        "competitive" => state.player_game(player).await,
        s => info!("Unknown game type : {}", s),
    };
}
