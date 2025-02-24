use axum::extract::ws::WebSocket;
// Circular queue
const MAX_PLAYERS : usize = 10;
pub struct Lobby {
    data : [Option<WebSocket> ; MAX_PLAYERS],
    front : usize,
    size : usize,
}
impl Lobby {
    pub fn new() -> Lobby {
        Lobby {
            data : [const { None } ; MAX_PLAYERS],
            front : 0,
            size : 0,
        }
    }

    pub fn enqueue(&mut self , element : WebSocket) {
        if self.size == MAX_PLAYERS {
            return;
        }

        let rear = (self.front + self.size) % MAX_PLAYERS;
        self.data[rear] = Some(element);
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<WebSocket> {
        if self.size == 0 {
            return None;
        }

        let element = self.data[self.front].take();
        self.data[self.front] = None;
        self.front = (self.front + 1) % MAX_PLAYERS;
        element
    }
    
    pub fn size(&self) -> usize {
        self.size
    }
}