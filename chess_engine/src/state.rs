pub struct State {
    castling_rights: CastlingRights,
}

impl State {
    pub fn new() -> Self {
        State {
            castling_rights: CastlingRights::new(),
        }
    }
}

// u8
// First 4 bits redundant
// Last 4 bits :
// 0 Black Queen
// 1 Black King
// 2 White Queen
// 3 White King
pub struct CastlingRights(u8);

impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights(0)
    }
}
