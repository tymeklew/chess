use axum::extract::ws::{Message, WebSocket};
use chess_engine::{bot_move, ChessGame, Square};
use futures::StreamExt;
use serde::Deserialize;

pub trait Game {

    async fn start(self);
}

pub struct PlayerGame {}
pub struct BotGame {
    game : ChessGame,
    sock : WebSocket
}

const BOT_DIFFICULTY : usize = 4;
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

#[derive(Deserialize)]
struct Communication {
    #[serde(rename="type")]
    _type : String,
    data : String,
}
//TODO
impl Game for BotGame {
    async fn start(mut self) {
        let (mut sender , mut receiver) = self.sock.split();

        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(command) = message {
                let communication: Communication = match serde_json::from_str(&command) {
                    Ok(comm) => comm,
                    Err(_) => continue
                };

                match communication._type.as_str() {
                    "move" => {
                        let mv = self.game.move_from_uci(&communication.data);
                        if let Some(mv) = mv {
                           self.game.mv(mv); 

                           let bot_mv =bot_move(&mut self.game.board(), BOT_DIFFICULTY, chess_engine::Sides::White).1.unwrap();
                            self.game.mv(bot_mv);
                        }


                    },
                    "draw" => {},
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
*/