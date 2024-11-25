use axum::extract::ws::WebSocket;
use chess_engine::PieceType;
use tokio::sync::mpsc::{Receiver, Sender};

pub enum GameMessage {
    Join(WebSocket),
}

pub struct Game {
    tx: Sender<GameMessage>,
    players: [Option<WebSocket>; 2],
}

impl Game {
    pub fn new(tx: Sender<GameMessage>) -> Self {
        Game {
            tx,
            players: [None, None],
        }
    }
    pub fn start(&self, mut rx: Receiver<GameMessage>) {
        let (socket1, socket2) = (self.players[0], self.players[1]);
        tokio::spawn(async move {
            tokio::select! {
                msg = rx.recv() => {}
            };
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
