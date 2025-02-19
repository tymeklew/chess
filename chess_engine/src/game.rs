use crate::board::Board;
use crate::Move;
pub struct ChessGame {
    turn: usize,
    board: Board,
}

impl ChessGame {
    pub fn new() -> Self {
        ChessGame {
            board: Board::new(),
            turn: 0,
        }
    }

    pub fn mv(&mut self, m: &Box<dyn Move>) {
        m.apply(&mut self.board);
        self.turn += 1;
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    // Corrolates the position of the pieces on the board to the bitboard for attack
}
