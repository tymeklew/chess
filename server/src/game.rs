use crate::player::Player;
use crate::Result;
use axum::extract::ws::{Message, WebSocket};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::io::Error;
use tokio::task::JoinHandle;
use uuid::Uuid;

#[derive(Debug)]
pub enum GameStatus {
    Ongoing,
    Draw,
    Winner(Uuid) 
}

#[derive(Serialize, Deserialize, Debug)]
struct GameEvent {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
}

#[derive(Debug)]
pub struct Game {
    draw_offered: Option<Uuid>,
    //Chat message to be sent id is the player who sent it
    chat: Option<(Uuid, String)>,
    status : GameStatus, 
}

pub type AxumMessageResult = Option<std::result::Result<Message, axum::Error>>;
impl Game {
    pub fn new() -> Self {
        Self {
            draw_offered: None,
            chat: None,
            status : GameStatus::Ongoing,
        }
    }

    pub fn start(mut self, mut p1: Player, mut p2: Player) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut players = [p1 , p2];


            loop {
                if let Some((id, txt)) = &self.chat {
                    for mut p in &players {
                        if p.id() == *id {
                            continue;
                        }

                    let data = serde_json::to_string(&GameEvent {
                        msg_type : "CHAT".into(),
                        data : Some(txt.into())
                }).unwrap();

                    p.sock().send(Message::Text(data)).await.unwrap();
            }

                    self.chat = None;
                }

                // Little fix for the tokio::select! macro
                let fix = players.split_at_mut(1);
                tokio::select! {
                    val = fix.0[0].sock().recv() => self.handle_message(val , players[0].id()).await.unwrap(),
                    val = fix.1[0].sock().recv() => self.handle_message(val , players[1].id()).await.unwrap(),
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
                    "DRAW_OFFER" => self.draw_offered = Some(id),
                    "DRAW_DECLINE" => self.draw_offered = None,
                    "DRAW_ACCEPT" => self.status = GameStatus::Draw,
                    _ => {}
                }
            }
        }

        Ok(())
    }
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
