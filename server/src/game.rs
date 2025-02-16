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
        todo!()
    }
}