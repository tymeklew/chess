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
use game::{Game, GameMessage};
use log::info;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::{self, channel, Receiver, Sender};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

const CHANNEL_BUFFER: usize = 100;

pub struct AppState {
    games: Vec<Game>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            games: Vec::with_capacity(10),
        }
    }
    // ->
    pub fn new_game(&mut self) -> Uuid {
        let (tx, rx) = mpsc::channel(100);
        let mut game = Game::new(tx);
        let id = game.id.clone();

        game.start(rx);
        self.games.push(game);

        id
    }

    pub async fn join(&mut self) -> (Sender<GameMessage>, Receiver<GameMessage>) {
        let mut available_games = Vec::new();
        for game in &self.games {
            if !*game.full.lock().await {
                available_games.push(game.id);
            }
        }

        if available_games.is_empty() {
            available_games.push(self.new_game());
        }

        let game = self
            .games
            .iter()
            .find(|f| &f.id == available_games.first().unwrap())
            .unwrap();

        let game_sender = game.tx.clone();
        let (tx, rx) = channel::<GameMessage>(CHANNEL_BUFFER);
        let _ = game_sender.send(GameMessage::Join(tx)).await;
        (game_sender, rx)
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

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
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<Mutex<AppState>>) {
    let (mut sender, mut reciever) = socket.split();
    let (tx, mut rx) = state.lock().await.join().await;

    tokio::spawn(async move {
        while let Some(rcv) = reciever.next().await {
            match rcv {
                Ok(msg) => match msg {
                    Message::Text(txt) => {
                        info!("Recieved : {}", txt);
                        let _ = tx.send(GameMessage::Text(txt)).await;
                    }
                    _ => {}
                },
                Err(e) => log::error!("Something went wrong with socket : {e}"),
            }
        }
    });

    while let Some(msg) = rx.recv().await {
        match msg {
            GameMessage::Text(txt) => {
                info!("Sending message to client");
                let _ = sender.send(Message::Text(txt)).await;
            }
            _ => {}
        }
    }
}
