use axum::extract::ws::{Message, WebSocket};
use chess_engine::{bot_move, ChessGame, GameStatus, Sides};
use futures::SinkExt;
use futures::StreamExt;
use log::info;
use serde::de::value::StringDeserializer;
use serde::{Deserialize, Serialize};
use sqlx::query;
use uuid::Uuid;

use crate::player::Player;
use crate::AppState;

pub trait Game {
    async fn start(self);
}

#[derive(Clone , Copy)]
pub enum BotDifficulty {
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

impl Communication{
    pub fn new(_type : String , data : Option<String>) -> Self {
        Self {
            _type,
            data,
        }
    }
}
//TODO
impl Game for BotGame {
    async fn start(mut self) {
        let (mut sender, mut receiver) = self.player.sock.split();
        info!("Hello");

        sender.send(Message::Text(WHITE_STARTED.clone())).await.unwrap();

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

                    if let GameStatus::Checkmate(_) = self.game.status() {
                        sender.send(Message::Text(serde_json::to_string(&Communication::new("game_over".to_string() , Some("checkmate,white".to_string()))).unwrap())).await.unwrap();
                        save_bot_game(&self.pool , &self.game , self.player.id , self.difficulty).await;
                        break;
                    }

                    info!("Calculating at difficulty {}" , self.difficulty as usize);
                    let mv = bot_move(&self.game.board(), self.difficulty as usize, self.side.other());
                    if mv.1.is_none() {
                        info!("Bot has no moves");
                        continue;
                    }
                    let temp = mv.1.unwrap();
                    self.game.mv(temp.clone());

                    let response = Communication {
                        _type: "move".to_string(),
                        data: Some(temp.into_uci(Sides::Black)),
                    };
                    let response = serde_json::to_string(&response).unwrap();
                    sender.send(Message::Text(response)).await.unwrap();

                    if let GameStatus::Checkmate(_) = self.game.status() {
                        sender.send(Message::Text(serde_json::to_string(&Communication::new("game_over".to_string() , Some("checkmate,black".to_string()))).unwrap())).await.unwrap();
                        save_bot_game(&self.pool , &self.game , self.player.id , self.difficulty).await;
                        break;
                    }
                },
                "legal_moves" => {
                    let legal_moves = self.game.legal_moves(self.side);
                    let legal_moves = legal_moves
                        .iter()
                        .map(|mv| mv.into_uci(Sides::White))
                        .collect::<Vec<String>>()
                        .join(",");
                    let response = Communication {
                        _type: "legal_moves".to_string(),
                        data: Some(legal_moves),
                    };
                    let response = serde_json::to_string(&response).unwrap();
                    info!("Responding with legal moves");
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

pub struct PlayerGame {
    game : ChessGame,
    white : Player,
    black : Player,
    pool : sqlx::PgPool,
}

impl PlayerGame {
    pub fn new(white : Player , black : Player , pool : sqlx::PgPool) -> Self {
        Self {
            game : ChessGame::new(),
            white,
            black,
            pool,
        }
    }
}

lazy_static::lazy_static! {
    static ref WHITE_STARTED : String = serde_json::to_string(&Communication::new("game_started".to_string() , Some("white".to_string()))).unwrap();
    static ref BLACK_STARTED : String = serde_json::to_string(&Communication::new("game_started".to_string() , Some("black".to_string()))).unwrap();
}

impl Game for PlayerGame {
     async fn start(mut self) {
        // Start time chrono
        let start = chrono::Utc::now();
        let (mut white_sender, mut white_receiver) = self.white.sock.split();
        let (mut black_sender, mut black_receiver) = self.black.sock.split();
        info!("Started for real");

        white_sender.send(Message::Text(WHITE_STARTED.clone())).await.unwrap();
        black_sender.send(Message::Text(BLACK_STARTED.clone())).await.unwrap();

        loop {
        tokio::select! {
            val = white_receiver.next() => {
                if val.is_none() {
                    return;
                }
                if let Some(Ok(Message::Text(msg))) = val {
                    let comm = serde_json::from_str::<Communication>(&msg).unwrap();

                    match comm._type.as_str() {
                        "legal_moves" => {
                            let mvs = legal_moves(&self.game , Sides::White);
                            white_sender.send(Message::Text(mvs)).await.unwrap();
                        },
                        "move" => {
                            info!("Got move");
                            let mv = comm.data.unwrap();
                            let mv = self.game.move_from_uci(&mv);

                            if mv.is_none() {
                                continue;
                            }
                            let mv = mv.unwrap();

                            self.game.mv(mv.clone());
                            let response = Communication {
                                _type : "move".to_string(),
                                data : Some(mv.into_uci(Sides::White)),
                            };
                            let response = serde_json::to_string(&response).unwrap();
                            black_sender.send(Message::Text(response)).await.unwrap();

                            if let GameStatus::Checkmate(_) = self.game.status() {
                                black_sender.send(Message::Text(serde_json::to_string(&Communication::new("game_over".to_string() , Some("checkmate,white".to_string()))).unwrap())).await.unwrap();
                                white_sender.send(Message::Text(serde_json::to_string(&Communication::new("game_over".to_string() , Some("checkmate,white".to_string()))).unwrap())).await.unwrap();
                                save_player_game(&self.pool , &self.game , self.white.id , self.black.id , start).await; 
                                break;
                            }
                        },
                        _ => {},
                    }

                }
            },
            val = black_receiver.next() => {
                if let Some(Ok(Message::Text(msg))) = val {
                    let comm = serde_json::from_str::<Communication>(&msg).unwrap();

                    match comm._type.as_str() {
                        "legal_moves" => {
                            let mvs = legal_moves(&self.game , Sides::Black);
                            black_sender.send(Message::Text(mvs)).await.unwrap();
                        },
                        "move" => {
                            let mv = comm.data.unwrap();
                            let mv = self.game.move_from_uci(&mv);

                            if mv.is_none() {
                                continue;
                            }
                            let mv = mv.unwrap();

                            self.game.mv(mv.clone());
                            let response = Communication {
                                _type : "move".to_string(),
                                data : Some(mv.into_uci(Sides::Black)),
                            };
                            let response = serde_json::to_string(&response).unwrap();
                            white_sender.send(Message::Text(response)).await.unwrap();
                            if let GameStatus::Checkmate(_) = self.game.status() {
                                black_sender.send(Message::Text(serde_json::to_string(&Communication::new("game_over".to_string() , Some("checkmate,black".to_string()))).unwrap())).await.unwrap();
                                white_sender.send(Message::Text(serde_json::to_string(&Communication::new("game_over".to_string() , Some("checkmate,black".to_string()))).unwrap())).await.unwrap();
                                save_player_game(&self.pool , &self.game , self.white.id , self.black.id , start).await; 
                                break;
                            }
                        }
                        _ => {},
                    }
                }
            },
        }
    }
    }
}

fn legal_moves(game : &ChessGame , side : Sides) -> String {
    let legal_moves = game.legal_moves(side);
    let mvs = legal_moves.iter().map(|mv| mv.into_uci(side)).collect::<Vec<String>>().join(",");
    let comm = Communication {
        _type : "legal_moves".to_string(),
        data : Some(mvs),
    };

    serde_json::to_string(&comm).unwrap()
}

const INSERT_PLAYER_GAME : &str = r#"
INSERT INTO player_games (game_id , white_id , black_id , started , finished , pgn , winner)
VALUES ($1 , $2 , $3 , $4 , $5 , $6 , $7)
"#;
pub async fn save_player_game(pool : &sqlx::PgPool , game : &ChessGame , white : Uuid , black : Uuid , started : chrono::DateTime<chrono::Utc>) {
    let game_id = Uuid::new_v4();
    let moves = game.all_moves();
    let winner_id = match game.status() {
        GameStatus::Checkmate(side) => {
            if side == Sides::White {
                white
            } else {
                black
            }
        },
        _ => return,
    };

    query(INSERT_PLAYER_GAME)
        .bind(game_id)
        .bind(white)
        .bind(black)
        .bind(started)
        .bind(chrono::Utc::now())
        .bind(moves)
        .bind(winner_id)
        .execute(pool)
        .await.unwrap();
}

const INSERT_BOT_GAME : &str = r#"
INSERT INTO bot_games (id , player_id , white , difficulty , png ,won)
VALUES ($1 , $2 , TRUE , $3 , $4 , $5)
"#;
pub async fn save_bot_game(pool : &sqlx::PgPool , game : &ChessGame , white : Uuid , difficulty : BotDifficulty) {
    let game_id = Uuid::new_v4();
    let moves = game.all_moves();
    let won = match game.status() {
        GameStatus::Checkmate(side) => {
            if side == Sides::Black {
                true
            } else {
                false
            }
        },
        _ => return,
    };


    query(INSERT_BOT_GAME)
        .bind(game_id)
        .bind(white)
        .bind(difficulty as i32)
        .bind(moves)
        .bind(won)
        .execute(pool)
        .await.unwrap();
}