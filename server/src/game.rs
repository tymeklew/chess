pub enum GameMessage {
    Text(String),
    Join(Sender<GameMessage>),
}
use log::info;
use tokio::sync::mpsc::{Receiver, Sender};
pub struct Game {
    //pub game: chess_engine::Game,
    pub tx: Sender<GameMessage>,
}

impl Game {
    pub fn start(&mut self, mut rx: Receiver<GameMessage>) {
        tokio::spawn(async move {
            let mut players = Vec::new();
            while let Some(msg) = rx.recv().await {
                match msg {
                    GameMessage::Join(sender) => players.push(sender),
                    GameMessage::Text(txt) => {
                        info!("Recieved message sending to : {}", players.len());
                        for p in players.iter() {
                            p.send(GameMessage::Text(txt.clone())).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
