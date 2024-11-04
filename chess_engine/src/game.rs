use crate::{
    board::Board,
    pieces::{self, Piece},
};

#[derive(Debug, Clone)]
pub struct Position {
    pub col: i8,
    pub row: i8,
}

impl Position {
    pub fn new(col: i8, row: i8) -> Self {
        Self { col, row }
    }
}

#[derive(Debug, Clone)]
pub enum Move {
    // Position of starting and ending position
    Basic(Position, Position),
    // No positional information needed as for a castle to happen both pieces need to be unmoved
    Castle(CastleType),
    EnPassant(Position, Position),
    // (Start position , End position , Type of piece to be promoted)
    Promotion(Position, Position, pieces::Piece),
}

impl Move {
    pub fn b(s_col: i8, s_row: i8, e_col: i8, e_row: i8) -> Self {
        Move::Basic(Position::new(s_col, s_row), Position::new(e_col, e_row))
    }
}

#[derive(Debug, Clone)]
pub enum CastleType {
    KingSide,
    QueenSide,
}

pub struct Game {
    pub board: Board,
    pub turn: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            turn: 0,
        }
    }

    pub fn mv(&mut self, mv: Move) -> bool {
        match mv.clone() {
            Move::Basic(start, end) => {
                self.board.moved(&start);
                self.board.set(&end, self.board.get(&start));
                self.board.set(&start, None);
            }
            _ => return false,
        }
        return true;
    }

    pub fn available_moves(&self, pos: Position) -> Option<Vec<Move>> {
        match self.board.get(&pos) {
            Some(piece) => Some(piece.available_moves(pos, &self.board)),
            None => None,
        }
    }
}
