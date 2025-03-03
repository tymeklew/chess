use axum::extract::ws::{Message, WebSocket};
use chess_engine::{bot_move, ChessGame, Sides, Square};
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use futures::SinkExt;

pub trait Game {

    async fn start(self);
}

pub struct PlayerGame {}
pub struct BotGame {
    game : ChessGame,
    sock : WebSocket
}

const BOT_DIFFICULTY : usize = 3;
impl BotGame {
    pub fn new(sock : WebSocket) -> Self {
        Self { sock , game : ChessGame::new() }
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

#[derive(Deserialize , Serialize , Debug)]
struct Communication {
    #[serde(rename="type")]
    _type : String,
    data : Option<String>,
}
//TODO
impl Game for BotGame {
    async fn start(mut self) {
        let (mut sender , mut receiver) = self.sock.split();

        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(command) = message {
                log::debug!("Received command: {}", command);
                let communication: Communication = match serde_json::from_str(&command) {
                    Ok(comm) => comm,
                    Err(_) => continue
                };
                log::debug!("Parsed command: {:?}", communication);

                match communication._type.as_str() {
                    "move" => {
                        let mv = self.game.move_from_uci(&communication.data.unwrap());
                        if let Some(mv) = mv {
                           self.game.mv(mv); 

                           let bot_mv = bot_move(&mut self.game.board(), BOT_DIFFICULTY, chess_engine::Sides::Black).1.unwrap();
                            self.game.mv(bot_mv.clone());

                            let response = Communication {
                                _type : "move".to_string(),
                                data : Some(bot_mv.into_uci(Sides::Black))
                            };

                            let response = serde_json::to_string(&response).unwrap();
                            sender.send(Message::Text(response)).await.unwrap();
                            println!("Send bot move");


                        }


                    },
                    "legal" => {
                        let mvs = self.game.legal_moves(Sides::White);
                        let legal_moves = mvs.iter().map(|m| m.into_uci(Sides::White)).collect::<Vec<String>>().join(",");

                        let response = Communication {
                            _type : "legal_moves".to_string(),
                            data : Some(legal_moves)
                        };

                        let response = serde_json::to_string(&response).unwrap();
                        sender.send(Message::Text(response)).await.unwrap();

                    }
                    _ => {},
                };


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