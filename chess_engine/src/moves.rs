use crate::board::{Bitboard, Board};
use crate::pieces::Pieces;
use crate::{Sides, Square};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum Move {
    Basic {
        source: Square,
        destination: Square,
    },
    Capture {
        source: Square,
        destination: Square,
        capture: Pieces,
    },
    Castling {
        side: Sides,
        king_side: bool,
    },
    Promotion {
        source: Square,
        destination: Square,
        capture: Option<Pieces>,
        promotion_piece: Pieces,
    },
    EnPassant,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Basic {
                source,
                destination,
            } => write!(f, "{} -> {}", source, destination),
            Move::Capture {
                source,
                destination,
                capture,
            } => write!(f, "{} -> {} capturing {}", source, destination, capture),
            Move::Promotion {
                source,
                destination,
                capture,
                promotion_piece,
            } => write!(
                f,
                "{} -> {} promoting to {}",
                source, destination, promotion_piece
            ),
            _ => todo!(),
        }
    }
}

impl Move {
    pub fn apply(&self, board: &mut Board) -> bool {
        match self {
            Move::Basic {
                source,
                destination,
            } => apply_basic(source, destination, board),
            Move::Capture {
                source,
                destination,
                capture,
            } => apply_capture(source, destination, capture, board),
            Move::Promotion {
                source,
                destination,
                capture,
                promotion_piece,
            } => apply_promotion(board, source, destination, capture, promotion_piece),
            _ => todo!(),
        }
    }
    pub fn undo(&self, board: &mut Board) -> bool {
        match self {
            Move::Basic {
                source,
                destination,
            } => undo_basic(source, destination, board),
            Move::Capture {
                source,
                destination,
                capture,
            } => undo_capture(source, destination, capture, board),
            Move::Promotion {
                source,
                destination,
                capture,
                promotion_piece,
            } => undo_promotion(board, source, destination, capture, promotion_piece),
            _ => todo!(),
        }
    }
    pub fn capture(&self) -> Option<Pieces> {
        match self {
            Move::Capture { capture, .. } => Some(*capture),
            Move::Promotion { capture, .. } => *capture,
            _ => None,
        }
    }
    pub fn into_uci(&self, side: Sides) -> String {
        match self {
            Move::Basic {
                source,
                destination,
            } => format!("{}{}", source, destination),
            Move::Capture {
                source,
                destination,
                ..
            } => format!("{}{}", source, destination),
            Move::Promotion {
                source,
                destination,
                promotion_piece,
                ..
            } => format!(
                "{}{}{}",
                source,
                destination,
                match side {
                    Sides::White => promotion_piece.to_uci().to_ascii_uppercase(),
                    Sides::Black => promotion_piece.to_uci().to_ascii_lowercase(),
                }
            ),
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

fn apply_capture(
    source: &Square,
    destination: &Square,
    capture: &Pieces,
    board: &mut Board,
) -> bool {
    // Remove captured piece
    let capture_side = match board.get_side(*destination) {
        Some(side) => side,
        None => return false,
    };

    board.sides[capture_side] ^= Bitboard(1 << destination.idx());
    board.pieces[capture_side][*capture] ^= Bitboard(1 << destination.idx());

    move_piece(source, destination, board);
    true
}
fn undo_capture(
    source: &Square,
    destination: &Square,
    capture: &Pieces,
    board: &mut Board,
) -> bool {
    //Add captured piece back
    let side = match board.get_side(*destination) {
        Some(side) => side,
        None => return false,
    };

    move_piece(destination, source, board);
    board.sides[side.other()] ^= Bitboard(1 << destination.idx());
    board.pieces[side.other()][*capture] ^= Bitboard(1 << destination.idx());
    true
}

fn apply_promotion(
    board: &mut Board,
    source: &Square,
    destination: &Square,
    capture: &Option<Pieces>,
    promotion_piece: &Pieces,
) -> bool {
    // if capture is Some , remove captured piece
    let side = match board.get_side(*source) {
        Some(side) => side,
        None => return false,
    };
    if capture.is_some() {
        board.sides[side.other()] ^= Bitboard(1 << destination.idx());
        board.pieces[side.other()][capture.unwrap()] ^= Bitboard(1 << destination.idx());
    }

    // Remove pawn
    board.sides[side] ^= Bitboard(1 << source.idx());
    board.pieces[side][Pieces::Pawn] ^= Bitboard(1 << source.idx());
    // Add promoted piece
    board.place_piece(side, *promotion_piece, *destination);
    true
}
fn undo_promotion(
    board: &mut Board,
    source: &Square,
    destination: &Square,
    capture: &Option<Pieces>,
    promotion_piece: &Pieces,
) -> bool {
    // Remove promoted piece
    let side = match board.get_side(*destination) {
        Some(side) => side,
        None => return false,
    };

    board.sides[side] ^= Bitboard(1 << destination.idx());
    board.pieces[side][*promotion_piece] ^= Bitboard(1 << destination.idx());

    board.place_piece(side, Pieces::Pawn, *source);

    if capture.is_some() {
        board.place_piece(side.other(), capture.unwrap(), *destination);
    }

    true
}

fn move_piece(source: &Square, destination: &Square, board: &mut Board) -> bool {
    let source_piece = match board.get_piece(*source) {
        Some(piece) => piece,
        None => return false,
    };
    let source_side = match board.get_side(*source) {
        Some(side) => side,
        None => return false,
    };

    board.sides[source_side] ^= Bitboard(1 << source.idx());
    board.pieces[source_side][source_piece] ^= Bitboard(1 << source.idx());

    board.place_piece(source_side, source_piece, *destination);
    true
}

// Convert to uci
