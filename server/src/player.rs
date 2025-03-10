use axum::extract::ws::WebSocket;
use uuid::Uuid;

pub struct Player {
    pub id: Uuid,
    pub sock: WebSocket,
}

impl Player {
    pub fn new(id : Uuid , sock : WebSocket) -> Self {
        Self {
            id,
            sock
        }
    }
}