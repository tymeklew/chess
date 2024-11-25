mod game;
mod player;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use futures::lock::Mutex;
use futures::{SinkExt, StreamExt};
use game::{game_to_fen, Game, GameMessage};
use log::info;
use serde_json::{json, Value};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CHANNEL_BUFFER_SIZE: usize = 100;

pub struct AppState {
    games: Vec<Game>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            games: Vec::with_capacity(10),
        }
    }
    pub fn new_game(&mut self) {
        let (tx, rx) = channel(CHANNEL_BUFFER_SIZE);
        let game = Game::new(tx);
        game.start(rx);

        self.games.push(game);
    }

    pub fn join(&mut self) {}
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => "8080".to_string(),
    };

    let state = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route("/ws", any(ws_handler))
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
    .await
    .unwrap();
}

async fn ws_handler(
    State(state): State<Arc<Mutex<AppState>>>,
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
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

async fn handle_socket(socket: WebSocket, addr: SocketAddr, state: Arc<Mutex<AppState>>) {}

/*
- Websocket communication
Sending a chat message
{
    type : "message",
    data : "Message"
}

{
    type : "game_state"
}
returns the game state

*/
