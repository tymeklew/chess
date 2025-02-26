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

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Basic { source, destination } => write!(f, "{} -> {}", source, destination),
            Move::Capture { source, destination, capture } => write!(f, "{} -> {} capturing {}", source, destination, capture),
            _ => todo!(),
        }
    }
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
    pub fn capture(&self) -> Option<Pieces> {
        match self {
            Move::Capture {capture , ..} => Some(*capture),
            _ => None,
        }
    }
    pub fn into_uci(&self) -> String {
        match self {
            Move::Basic { source, destination } => format!("{}{}", source, destination),
            Move::Capture { source, destination, .. } => format!("{}{}", source, destination),
            _ => todo!(),
        }
    }
}

fn apply_basic(source: &Square, destination: &Square, board: &mut Board) -> bool {
    move_piece(source, destination, board)

}
fn undo_basic(source: &Square, destination: &Square, board: &mut Board) -> bool {
   move_piece(destination, source, board) 
}

fn apply_capture(source: &Square, destination: &Square, capture: &Pieces, board: &mut Board) -> bool {
    // Remove captured piece
    let capture_side = match board.get_side(*destination) {
        Some(side) => side,
        None => return false
    };
    let capture_piece = match board.get_piece(*destination) {
        Some(piece) => piece,
        None => return false
    };

    board.sides[capture_side] ^= Bitboard(1 << destination.idx());
    board.pieces[capture_side][capture_piece] ^= Bitboard(1 << destination.idx());

    move_piece(source, destination, board);
    false
}
fn undo_capture(source: &Square, destination: &Square, capture: &Pieces, board: &mut Board) -> bool {
    //Add captured piece back 
    let capture_side = match board.get_side(*destination) {
        Some(side) => side,
        None => return false
    };

    board.sides[capture_side] ^= Bitboard(1 << destination.idx());
    board.pieces[capture_side][*capture] ^= Bitboard(1 << destination.idx());

    move_piece(destination, source, board);
    true
}

fn move_piece(source : &Square , destination: &Square , board : &mut Board) -> bool {
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

    // Convert to uci
