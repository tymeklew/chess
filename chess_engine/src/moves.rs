use crate::{board::{Bitboard, Board}, pieces::{self, Pieces, Sides}, square::Square};
use std::{fmt::{Debug, Display}, ops::BitAnd};

pub trait Move: Display + Debug  {
    fn apply(&self, board: &mut Board);
    fn undo(&self) -> Box<dyn Move>;
    fn capture(&self) -> Option<Pieces>;
}

#[derive(Clone, Debug)]
pub struct BasicMove {
    from: Square,
    to: Square,
}
impl Display for BasicMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

impl Move for BasicMove {
    fn apply(&self, board: &mut Board) {
        let side = board.get_side(self.from);
        let piece = board.get_piece(self.from);

        board.sides[side].0 ^= 1 << self.from.idx(); // Remove piece from the source square
        board.sides[side].0 ^= 1 << self.to.idx();   // Add piece to the destination square
        
        board.pieces[side][piece].0 ^= 1 << self.from.idx(); // Remove from piece-specific bitboard
        board.pieces[side][piece].0 ^= 1 << self.to.idx();   // Add to piece-specific bitboard
    }
    fn undo(&self) -> Box<dyn Move> {
        todo!()
    }
    fn capture(&self) -> Option<Pieces> {
        None
    }
}

impl BasicMove {
    pub fn new(from: Square, to: Square) -> Self {
        BasicMove { from, to }
    }
}

#[derive(Clone, Debug)]
pub struct Capture {
    from: Square,
    to: Square,
    capture: Pieces,
}

impl Display for Capture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} capturing {}", self.from, self.to, self.capture)
    }
}

impl Move for Capture {
    fn apply(&self, board: &mut Board) {
        let capture_side = board.get_side(self.to);
        let capture_piece = board.get_piece(self.to);

        board.sides[capture_side] ^= Bitboard(1 << self.to.idx());
        board.pieces[capture_side][capture_piece] ^= Bitboard(1 << self.to.idx());
        
        BasicMove::new(self.from, self.to).apply(board);
    }

    fn undo(&self) -> Box<dyn Move> {
        todo!()
    }

    fn capture(&self) -> Option<Pieces> {
        Some(self.capture)
    }
}

impl Capture {
    pub fn new(from: Square, to: Square, capture: Pieces) -> Self {
        Capture { from, to, capture }
    }
}
