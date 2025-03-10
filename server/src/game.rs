use axum::extract::ws::{Message, WebSocket};
use chess_engine::{bot_move, ChessGame, GameStatus, Sides};
use futures::SinkExt;
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};

use crate::player::Player;
use crate::AppState;

pub trait Game {
    async fn start(self);
}

#[derive(Clone , Copy)]
enum BotDifficulty {
    Easy = 1,
    Medium = 2,
    Hard = 3,
}

pub struct BotGame {
    game: ChessGame,
    player : Player,
    pool : sqlx::PgPool,
    difficulty : BotDifficulty,
    side : Sides,
}

impl BotGame {
    pub fn new(player: Player , pool : sqlx::PgPool , difficulty : String , side : String) -> Self {
        let difficulty = match difficulty.as_str() {
            "easy" => BotDifficulty::Easy,
            "medium" => BotDifficulty::Medium,
            "hard" => BotDifficulty::Hard,
            _ => BotDifficulty::Easy,
        };
        let side = match side.as_str() {
            "white" => Sides::White,
            "black" => Sides::Black,
            _ => Sides::White,
        };
        Self {
            player,
            game: ChessGame::new(),
            pool,
            difficulty,
            side,
        }
    }
}

/*
   // We subscribe *before* sending the "joined" message, so that we will also
   // display it to our client.
   let mut rx = state.tx.subscribe();

   // Now send the "joined" message to all subscribers.
   let msg = format!("{username} joined.");
   tracing::debug!("{msg}");
   let _ = state.tx.send(msg);
*/

#[derive(Deserialize, Serialize, Debug)]
struct Communication {
    #[serde(rename = "type")]
    _type: String,
    data: Option<String>,
}
//TODO
impl Game for BotGame {
    async fn start(mut self) {
        let (mut sender, mut receiver) = self.player.sock.split();

        while let Some(Ok(msg)) = receiver.next().await {
            if self.game.status() != GameStatus::InProgress {
                break;
            }

            let msg = match msg {
                Message::Text(text) => text,
                _ => continue,
            };

            let communication: Communication = match serde_json::from_str(&msg) {
                Ok(communication) => communication,
                Err(_) => continue,
            };

            match communication._type.as_str() {
                "move" => {
                    let mv = communication.data.unwrap();
                    let mv = self.game.move_from_uci(&mv);

                    if mv.is_none() {
                        continue;
                    }

                    self.game.mv(mv.unwrap());
                    let mv = bot_move(&self.game.board(), self.difficulty as usize, self.side);
                    if mv.1.is_none() {
                        continue;
                    }
                    self.game.mv(mv.1.unwrap());
                },
                "legal_moves" => {
                    let legal_moves = self.game.legal_moves(self.side);
                    let legal_moves = legal_moves
                        .iter()
                        .map(|mv| mv.into_uci(self.side))
                        .collect::<Vec<String>>()
                        .join(",");
                    let response = Communication {
                        _type: "legal_moves".to_string(),
                        data: Some(legal_moves),
                    };
                    let response = serde_json::to_string(&response).unwrap();
                    sender.send(Message::Text(response)).await.unwrap();
                }
                _ => continue,
            }

        }

    }
}

// UCI (Universal Chess Interface)
/*
   Make Move
   type : "move"
   data : "e5e6"

   Legal Moves
   type : "legal_moves"
   data : "e4e5,"

   type : "game_over"
   // How the game ended , checkmate, stalemate, draw
   // Who won , white, black
   data : "checkmate,white"
*/
