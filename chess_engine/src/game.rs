use crate::board::Board;
use crate::pieces::Pieces;
use crate::{Move, Sides, Square};
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

    pub fn mv<T>(&mut self, m: &T) where T : Move {
        m.apply(&mut self.board);
        self.turn = self.turn.other();
    }

    pub fn boxed_mv(&mut self, m: Box<dyn Move>) {
        m.apply(&mut self.board);
        self.turn = self.turn.other();
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

}
