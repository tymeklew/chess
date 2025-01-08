use std::io::Error;
use tokio::task::JoinHandle;
use log::info;
use axum::extract::ws::{Message, WebSocket};
use crate::player::Player;
use crate::Result;
use serde::{Serialize , Deserialize};


#[derive(Serialize , Deserialize)]
struct GameEvent {
    #[serde(rename = "type")]
    msg_type : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data : Option<String>
}

pub struct Game {
}

impl Game {
    pub fn start(mut self , mut p1 : Player ,mut p2 : Player) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    val = p1.sock().recv() => {
                        if let Some(Ok(Message::Text(txt))) = val {
                            self.handle_message(txt , &mut p2);
                        }
                    }
                    val = p2.sock().recv() => {
                        if let Some(Ok(Message::Text(txt))) = val {
                            broadcast_message(&mut p1, txt).await;
                        }
                    }
                }
            }   
        })
    } 
    async fn handle_message(&mut self , msg :String , to : &mut Player) -> Result<()> {
        let evt : GameEvent = serde_json::from_str(&msg)?;
        match evt.msg_type.as_str() {
            "CHAT" => broadcast_message(to, evt.data.unwrap_or(String::new())).await?,
            "MOVE" => (),
            "RESIGN" => (),
            "DRAW" => (),
            _ => {},
        };
        Ok(())
}


}

async fn broadcast_message(to : &mut Player , msg : String) -> Result<()> {
    Ok(to.sock().send(Message::Text(msg)).await?)
} 


const Chat : usize = 0;
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
 */

