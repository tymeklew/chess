use axum::extract::ws::WebSocket;
use uuid::Uuid;
pub struct Player {
    id : Uuid,
    sock : WebSocket
}

impl Player {
    pub fn new(sock : WebSocket) -> Self {
        Player { id : Uuid::new_v4(), sock }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn sock(&mut self) -> &mut WebSocket {
        &mut self.sock
    }

}