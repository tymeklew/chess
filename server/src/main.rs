mod game;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use futures::lock::Mutex;
use futures::SinkExt;
use futures::StreamExt;
use game::{Game, GameMessage};
use log::info;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppState {
    games: HashMap<u8, Game>,
}

impl AppState {
    // ->
    pub fn new_game(&mut self) {
        let (tx, rx) = mpsc::channel(100);
        let mut game = Game { tx };
        game.start(rx);
        self.games.insert(0, game);
    }

    // -> (Send to game , Recieve from game)
    pub async fn join(&mut self) -> (Sender<GameMessage>, Receiver<GameMessage>) {
        if self.games.len() < 1 {
            self.new_game();
        }
        // Sender for the game
        let game_sender = self.games.get(&0_u8).unwrap().tx.clone();
        let (tx, rx) = mpsc::channel(100);

        game_sender.send(GameMessage::Join(tx)).await.unwrap();

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

    let state = Arc::new(Mutex::new(AppState {
        games: HashMap::new(),
    }));
    //let cors = CorsLayer::new();

    let app = Router::new()
        .route("/ws", any(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
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

async fn handle_socket(socket: WebSocket, addr: SocketAddr, state: Arc<Mutex<AppState>>) {
    let (mut sender, mut reciever) = socket.split();
    let (tx, mut rx) = state.lock().await.join().await;

    if sender.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {addr}");
    } else {
        return;
    }

    tokio::spawn(async move {
        while let Some(Ok(msg)) = reciever.next().await {
            match msg {
                Message::Text(txt) => tx.send(GameMessage::Text(txt)).await.unwrap(),
                _ => {}
            }
        }
    });

    while let Some(msg) = rx.recv().await {
        match msg {
            GameMessage::Text(txt) => {
                let _ = sender.send(Message::from(txt)).await;
            }
            _ => {}
        }
    }
}
