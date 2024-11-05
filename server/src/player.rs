use crate::game::GameMessage;
use axum::extract::ws::WebSocket;
use futures::{channel::mpsc::Sender, StreamExt};
use log::info;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

pub struct Player {
    pub id: Uuid,
    pub rx: Receiver<GameMessage>,
    pub tx : Sender<GameMessage>
    pub socket: WebSocket,
}

impl Player {
    pub fn new(rx: Receiver<GameMessage>, tx : Sender<GameMessage>, socket: WebSocket) -> Self {
        Self {
            id: Uuid::new_v4(),
            rx,
            tx,
            socket,
        }
    }

    pub fn start(self) {
        let (sender, reciever) = self.socket.split();
        tokio::spawn(async move { Player::writer() });
        tokio::spawn(async move { Player::reader() });
    }

    // Sends information to game
    pub async fn writer() {
        info!("Writer");
    }
    // Sends information to player
    pub async fn reader() {
        info!("Reader");
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
