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
use game::Game;
use log::info;
use player::Player;
use serde_json::{json, Value};
use tokio::task::JoinHandle;
use uuid::Uuid;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CHANNEL_BUFFER_SIZE: usize = 100;
pub type Result<T> = std::result::Result<T , Box<dyn std::error::Error>>;

pub struct AppState {
    lobby : HashMap<Uuid , Player>,
    games : Vec<JoinHandle<()>>
}

impl AppState {
    pub fn join(&mut self , sock : WebSocket) {
        let player = Player::new(sock);
        let id = player.id();

        self.lobby.insert(id, player);
        self.compatible(id);
    }

    // Find compatible opponent for the last player that joined
    pub fn compatible(&mut self , id : Uuid) {
        for player in &mut self.lobby.values() {
            if player.id() != id {
                info!("Found match");
                let  p1 = self.lobby.remove(&player.id()).unwrap();
                let p2 = self.lobby.remove(&id);

                self.start(p1 , p2.unwrap());
                break;
            }
        }
    }

    // Start a new game with the players from that index
    pub fn start(&mut self , p1 : Player  , p2 : Player) {
        Game::start(p1, p2);
    }
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

    //TODO New game implementation
    let state = Arc::new(Mutex::new(AppState {
        lobby : HashMap::new(),
        games : Vec::new(),
    }));

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

async fn handle_socket(mut sock: WebSocket, addr: SocketAddr, state: Arc<Mutex<AppState>>) {
    if sock.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
    } else {
        println!("Could not send ping {addr}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }
    state.lock().await.join(sock);
}