use crate::game::GameMessage;
use axum::extract::ws::{Message, WebSocket};
use futures::{
    channel::mpsc::Sender,
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use log::{error, info};
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

pub struct Player {
    pub id: Uuid,
    pub rx: Receiver<GameMessage>,
    pub tx: Sender<GameMessage>,
}

impl Player {
    pub fn new(rx: Receiver<GameMessage>, tx: Sender<GameMessage>, socket: WebSocket) -> Self {
        Self {
            id: Uuid::new_v4(),
            rx,
            tx,
        }
    }

    pub fn start(&self, socket: WebSocket) {
        let (sender, reciever) = socket.split();
        tokio::spawn(async move { Player::writer(sender , self.rx) });
        tokio::spawn(async move { Player::reader(reciever) });
    }

    // Sends information to the player reciever from game
    pub async fn writer(sender: SplitSink<WebSocket, Message> , reciever : Receiver<GameMessage>) {
        info!("Writer");
    }
    // Sends information to the game reciever from player
    pub async fn reader(mut reciever: SplitStream<WebSocket>) {
        while let Some(rcv) = reciever.next().await {
            match rcv {
                Ok(msg) => match msg {
                    GameMessage::Text(txt) =>
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
