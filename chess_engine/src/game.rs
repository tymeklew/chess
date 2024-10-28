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

#[derive(Debug, Clone)]
pub enum CastleType {
    KingSide,
    QueenSide,
}

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
        }
    }

    pub fn mv(&mut self, mv: Move) -> bool {
        match mv {
            Move::Basic(start, end) => {
                self.board.set(end, self.board.get(start.clone()));
                self.board.set(start, None);
                /* TODO
                Make it so that when the piece is moved the has_moved value is updated
                */
            }
            _ => {}
        }
        return true;
    }
}
