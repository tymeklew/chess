use crate::{player::Player, CHANNEL_BUFFER};
use futures::lock::Mutex;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use uuid::Uuid;

pub enum GameMessage {
    Text(String),
    Move(chess_engine::Move),
    Join(Sender<GameMessage>),
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
            let mut players = Vec::new();
            while let Some(msg) = rx.recv().await {
                match msg {
                    GameMessage::Join(tx) => {
                        players.push(tx);
                    }
                    GameMessage::Text(txt) => {
                        for p in &players {
                            let _ = p.send(GameMessage::Text(txt.clone())).await;
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}
