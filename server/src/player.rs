use crate::game::GameMessage;
use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use log::{error, info};
use tokio::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;

pub struct Player {
    pub id: Uuid,
    pub tx: Sender<GameMessage>,
}

impl Player {
    pub fn new(tx: Sender<GameMessage>, socket: WebSocket) -> Self {
        Self {
            id: Uuid::new_v4(),
            tx,
        }
    }

    pub fn start(&self, socket: WebSocket, rx: Receiver<GameMessage>) {
        let (sender, reciever) = socket.split();
        let tx = self.tx.clone();
        tokio::spawn(async move { Player::writer(sender, rx) });
        tokio::spawn(async move { Player::reader(reciever, tx) });
    }

    // Sends information to the player reciever from game
    pub async fn writer(
        sender: SplitSink<WebSocket, Message>,
        mut reciever: Receiver<GameMessage>,
    ) {
        info!("Writer");
    }
    // Sends information to the game reciever from player
    pub async fn reader(mut reciever: SplitStream<WebSocket>, sender: Sender<GameMessage>) {
        while let Some(rcv) = reciever.next().await {
            match rcv {
                Ok(msg) => match msg {
                    Message::Text(txt) => {}
                    _ => {}
                },
                Err(e) => error!("Something went wrong with socket : {}", e),
            }
        }
        info!("Reader");
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
