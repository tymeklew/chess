use crate::player::Player;
use crate::Result;
use axum::extract::ws::{Message, WebSocket};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::io::Error;
use tokio::task::JoinHandle;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct GameEvent {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
}

pub struct Game {
    draw_offered: Option<Uuid>,
    //Chat message to be sent id is the player who sent it
    chat: Option<(Uuid, String)>,
}

pub type AxumMessageResult = Option<std::result::Result<Message, axum::Error>>;
impl Game {
    pub fn new() -> Self {
        Self {
            draw_offered: None,
            chat: None,
        }
    }

    pub fn start(mut self, mut p1: Player, mut p2: Player) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if let Some((id, txt)) = self.chat {
                    match id {
                        _ if id == p1.id() => broadcast_message(&mut p2, txt).await.unwrap(),
                        _ if id == p2.id() => broadcast_message(&mut p1, txt).await.unwrap(),
                        _ => {}
                    }
                    self.chat = None;
                }

                tokio::select! {
                    val = p1.sock().recv() => self.handle_message(val , p1.id()).await.unwrap(),
                    val = p2.sock().recv() => self.handle_message(val , p2.id()).await.unwrap(),
                };
            }
        })
    }

    async fn handle_message(&mut self, msg: AxumMessageResult, id: Uuid) -> Result<()> {
        if let None = msg {
            return Ok(());
        }

        if let Message::Text(txt) = msg.unwrap()? {
            if let Ok(evt) = serde_json::from_str::<GameEvent>(&txt) {
                println!("Event : {:?}", evt);
                match evt.msg_type.as_str() {
                    "CHAT" => self.chat = Some((id, txt)),
                    "MOVE" => {}
                    "RESIGN" => {}
                    "DRAW_OFFER" => {}
                    "DRAW_DECLINE" => {}
                    "DRAW_ACCEPT" => {}
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

async fn broadcast_message(to: &mut Player, msg: String) -> Result<()> {
    Ok(to.sock().send(Message::Text(msg)).await?)
}

const Chat: usize = 0;
/* JSON communication
Chat message
{
    "type" : "CHAT",
    "data" : "Hello World"
}
Move
{
    "type" : "MOVE",
    "data" : "Be5"
}
Resign
{
    "type" : "RESIGN"
}
Draw {
    "type" : "DRAW_OFFER/DRAW_ACCEPT/DRAW_DECLINE"
}
GameEnd
{
    "type" : "GAME_OVER",
    "data" : "WINNER/LOSER"
}
 */
