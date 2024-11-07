use crate::{player::Player, CHANNEL_BUFFER};
use chess_engine::{Colour, PieceType, Position};
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
    pub tx: Sender<GameMessage>,
    pub players: Arc<Mutex<Vec<Player>>>,
    pub full: Arc<Mutex<bool>>,
}

impl Game {
    pub fn new(tx: Sender<GameMessage>) -> Self {
        Self {
            id: Uuid::new_v4(),
            tx,
            players: Arc::new(Mutex::new(Vec::with_capacity(2))),
            full: Arc::new(Mutex::new(false)),
        }
    }
    pub fn start(&mut self, mut rx: Receiver<GameMessage>) {
        let full = Arc::clone(&self.full);
        tokio::spawn(async move {
            let game = chess_engine::Game::new();
            let mut players = Vec::new();
            while let Some(msg) = rx.recv().await {
                match msg {
                    GameMessage::Join(tx) => {
                        if players.len() >= 2 {
                            *full.lock().await = true;
                            continue;
                        }
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

// p,R,Kn,B,Q,K
// p11W;
pub fn convert_game_to_json(board: chess_engine::Board) -> String {
    let mut total = String::new();
    for i in 0..8 {
        for j in 0..8 {
            match board.get(&Position::new(i, j)) {
                None => continue,
                Some(piece) => {
                    let mut str = String::new();
                    str += match piece.piece_type() {
                        PieceType::Pawn => "p",
                        PieceType::Rook => "r",
                        PieceType::Knight => "kn",
                        PieceType::Bishop => "b",
                        PieceType::Queen => "q",
                        PieceType::King => "k",
                    };

                    str += &i.to_string();
                    str += &j.to_string();
                    str += match piece.colour() {
                        Colour::White => "w",
                        Colour::Black => "b",
                    };
                    str += ";";

                    total += &str;
                }
            }
        }
    }
    total
}
