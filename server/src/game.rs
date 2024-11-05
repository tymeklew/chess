use crate::{player::Player, CHANNEL_BUFFER};
use axum::extract::ws::WebSocket;
use futures::lock::Mutex;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use uuid::Uuid;

pub enum GameMessage {
    Text(String),
    Move(chess_engine::Move),
    Join(WebSocket),
}

pub struct Game {
    pub id: Uuid,
    //pub game: chess_engine::Game,
    pub tx: Sender<GameMessage>,
    pub players: Arc<Mutex<Vec<Player>>>,
}

impl Game {
    pub fn new(tx: Sender<GameMessage>) -> Self {
        Self {
            id: Uuid::new_v4(),
            tx,
            players: Arc::new(Mutex::new(Vec::with_capacity(2))),
        }
    }
    pub fn start(&mut self, mut rx: Receiver<GameMessage>) {
        tokio::spawn(async move {
            //let mut players = Vec::new();
            while let Some(msg) = rx.recv().await {
                match msg {
                    GameMessage::Join(socket) => {}
                    GameMessage::Text(txt) => {}
                    _ => {}
                }
            }
        });
    }
}
