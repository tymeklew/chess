use crate::board::Board;
use crate::{Move, Sides};
pub struct ChessGame {
    turn: Sides,
    board: Board,
}

impl ChessGame {
    pub fn new() -> Self {
        ChessGame {
            board: Board::new(),
            turn: Sides::White,
        }
    }

    pub fn mv(&mut self, m: Move)
    {
        m.apply(&mut self.board);
        self.turn = self.turn.other();
    }
    pub fn move_from_uci(&self, input: &str) -> Option<Move>
    {
        self.board.move_from_uci(input)
    }
    pub fn board(&self) -> &Board {
        &self.board
    }
}
