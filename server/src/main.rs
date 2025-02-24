mod auth;
mod error;
mod friends;
mod game;
mod player;
mod lobby;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::{any, get, post};
use axum::{Extension, Router};
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::game::{BotGame, Game};
use crate::lobby::Lobby;

pub struct AppState {
    pool: sqlx::PgPool,
    lobby : Mutex<Lobby>,
}

impl AppState {
    pub fn new(pool: sqlx::PgPool, lobby: Lobby) -> Self {
        Self { pool, lobby : Mutex::new(lobby)}
    }

    pub fn add_player(&self , sock : WebSocket) {
        self.lobby.lock().unwrap().enqueue(sock);

        let sock = self.lobby.lock().unwrap().dequeue().unwrap();
        let game = BotGame::new(sock);

        tokio::spawn(async move { game.start().await });
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
    let state = Arc::new(AppState::new(pool , Lobby::new()));

    let app = Router::new()
        .route("/api/bot", any(ws_handler))
        .route("/api/friends/request", post(friends::friend_request))
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

async fn ws_handler(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    info!("{user_agent} connected at {addr}");
    ws.on_upgrade(move |sock| handle_socket(sock, addr, state))
}

async fn handle_socket(mut sock: WebSocket, addr: SocketAddr, state: Arc<AppState>) {
    if sock.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
    } else {
        println!("Could not send ping {addr}!");
        return;
    }

    state.add_player(sock);
}
