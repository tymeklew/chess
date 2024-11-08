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
    RequestGameState,
    GameState(String),
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
                        for p in &players {
                            let _ = p
                                .send(GameMessage::GameState(game_to_fen(game.board)))
                                .await;
                        }
                    }
                    GameMessage::Text(txt) => {
                        for p in &players {
                            let _ = p.send(GameMessage::Text(txt.clone())).await;
                        }
                    }
                    GameMessage::RequestGameState => {
                        for p in &players {
                            let _ = p
                                .send(GameMessage::GameState(game_to_fen(game.board)))
                                .await;
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
pub fn game_to_fen(board: chess_engine::Board) -> String {
    let mut str = String::new();
    let mut count = 0;
    for row in board.full() {
        if row.iter().all(|f| f.is_none()) {
            str += "8/";
            continue;
        }

        for item in row {
            match item {
                Some(piece) => {
                    if count != 0 {
                        str += &count.to_string();
                        count = 0;
                    }

                    str += match piece.piece_type() {
                        PieceType::Pawn => "p",
                        PieceType::Knight => "n",
                        PieceType::Rook => "r",
                        PieceType::Bishop => "b",
                        PieceType::Queen => "q",
                        PieceType::King => "k",
                    }
                }
                None => count += 1,
            }
        }
        str += "/";
        count = 0;
    }

    str
}
