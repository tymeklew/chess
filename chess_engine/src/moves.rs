use crate::board::{Bitboard, Board};
use crate::pieces::Pieces;
use crate::{Sides, Square};
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Move {
    Basic { source: Square, destination: Square },
    Capture { source: Square, destination: Square, capture: Pieces },
    Castling { side: Sides, king_side: bool },
    Promotion { source: Square, destination: Square, capture: Option<Pieces>, promotion_piece: Pieces },
    EnPassant,
}

impl Move {
    pub fn apply(&self, board: &mut Board) -> bool {
        match self {
            Move::Basic { source, destination } => apply_basic(source, destination, board),
            Move::Capture { source, destination, capture } => apply_capture(source, destination, capture, board),
            _ => todo!()
        }
    }
    pub fn undo(&self, board: &mut Board) -> bool {
        match self {
            Move::Basic { source, destination } => undo_basic(source, destination, board),
            Move::Capture { source, destination, capture } => undo_capture(source, destination, capture, board),
            _ => todo!()
        }
    }
}

fn apply_basic(source: &Square, destination: &Square, board: &mut Board) -> bool {
    let source_piece = match board.get_piece(*source) {
        Some(piece) => piece,
        None => return false
    };
    let source_side = match board.get_side(*source) {
        Some(side) => side,
        None => return false
    };

    board.sides[source_side] ^= Bitboard(1 << source.idx());
    board.pieces[source_side][source_piece] ^= Bitboard(1 << source.idx());

    board.place_piece(source_side, source_piece, *destination);
    true
}
fn undo_basic(source: &Square, destination: &Square, board: &mut Board) -> bool {
    false
}
fn apply_capture(source: &Square, destination: &Square, capture: &Pieces, board: &mut Board) -> bool {
    false
}
fn undo_capture(source: &Square, destination: &Square, capture: &Pieces, board: &mut Board) -> bool {
    false
}

/*pub trait Move: Display + Debug {
    fn apply(&self, board: &mut Board);
    fn undo(&self, board: &mut Board);
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
        board.sides[side].0 ^= 1 << self.to.idx(); // Add piece to the destination square

        board.pieces[side][piece].0 ^= 1 << self.from.idx(); // Remove from piece-specific bitboard
        board.pieces[side][piece].0 ^= 1 << self.to.idx(); // Add to piece-specific bitboard
    }
    fn undo(&self, board: &mut Board) {
        let side = board.get_side(self.to);
        let piece = board.get_piece(self.to);

        board.sides[side] ^= Bitboard(1 << self.to.idx());
        board.pieces[side][piece] ^= Bitboard(1 << self.to.idx());

        board.place_piece(side, piece, self.from);
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
pub struct Promotion {
    from: Square,
    to: Square,
    promotion: Pieces,
}

impl Promotion {
    pub fn new(from: Square, to: Square, promotion: Pieces) -> Self {
        Promotion {
            from,
            to,
            promotion,
        }
    }
}

impl Display for Promotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} promoting to {}",
            self.from, self.to, self.promotion
        )
    }
}

impl Move for Promotion {
    fn apply(&self, board: &mut Board) {
        todo!()
    }

    fn capture(&self) -> Option<Pieces> {
        None
    }

    fn undo(&self, board: &mut Board) {
        todo!()
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

    fn undo(&self, board: &mut Board) {
        let side = board.get_side(self.to);
        let piece = board.get_piece(self.to);
        // Remove captured piece from board
        // Replace captured piece to the destination square
        // Add capture piece to the source square:w

        board.sides[side] ^= Bitboard(1 << self.to.idx());
        board.pieces[side][piece] ^= Bitboard(1 << self.to.idx());

        board.place_piece(side.other(), self.capture, self.to);
        board.place_piece(side, piece, self.from);
    }

    fn capture(&self) -> Option<Pieces> {
        Some(self.capture)
    }
}

impl Capture {
    pub fn new(from: Square, to: Square, capture: Pieces) -> Self {
        Capture { from, to, capture }
    }
}*/
