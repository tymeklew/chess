use crate::board::{Bitboard, Board};
use crate::game::{BLACK_KING, BLACK_KING_SIDE_ROOK, BLACK_QUEEN_SIDE_ROOK, WHITE_KING, WHITE_KING_SIDE_ROOK, WHITE_QUEEN_SIDE_ROOK};
use crate::pieces::Pieces;
use crate::{Sides, Square};
use std::fmt::{Debug, Display};

pub const WHITE_QUEEN_CASTLING_UCI: &str = "e1c1";
pub const WHITE_KING_CASTLING_UCI: &str = "e1g1";
pub const BLACK_QUEEN_CASTLING_UCI: &str = "e8c8";
pub const BLACK_KING_CASTLING_UCI: &str = "e8g8";

#[derive(Clone, Debug)]
pub enum CastlingSide {
    KingSide,
    QueenSide,
}

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
        castling_side: CastlingSide,
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
            Move::Castling {
                side,
                castling_side,
            } => write!(
                f,
                "{} castling {}",
                match side {
                    Sides::White => "white",
                    Sides::Black => "black",
                },
                match castling_side {
                    CastlingSide::KingSide => "king side",
                    CastlingSide::QueenSide => "queen side",
                }
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
            Move::Castling { side, castling_side } => match (side ,  castling_side) {
                (Sides::White , CastlingSide::QueenSide) => {
                    move_piece(&WHITE_KING , &Square::new(2 , 0) , board) && 
                    move_piece(&WHITE_QUEEN_SIDE_ROOK , &Square::new(3 , 0) , board)
                },
                (Sides::White , CastlingSide::KingSide) => {
                    move_piece(&WHITE_KING , &Square::new(6 , 0) , board) && 
                    move_piece(&WHITE_KING_SIDE_ROOK , &Square::new(5 , 0) , board)
                },
                (Sides::Black , CastlingSide::QueenSide) => {
                    move_piece(&BLACK_KING, &Square::new(2 , 7) , board) && 
                    move_piece(&BLACK_QUEEN_SIDE_ROOK , &Square::new(3 , 7) , board)
                },
                (Sides::Black , CastlingSide::KingSide) => {
                    move_piece(&BLACK_KING , &Square::new(6 , 7) , board) && 
                    move_piece(&BLACK_KING_SIDE_ROOK , &Square::new(5 , 7) , board)
                },
            }
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
            Move::Castling { side, castling_side } => match (side , castling_side) {
                (Sides::White , CastlingSide::QueenSide) => {
                    move_piece(&Square::new(2 , 0) , &WHITE_KING , board) && 
                    move_piece(&Square::new(3 , 0) , &WHITE_QUEEN_SIDE_ROOK , board)
                },
                (Sides::White , CastlingSide::KingSide) => {
                    move_piece(&Square::new(6 , 0) , &WHITE_KING , board) && 
                    move_piece(&Square::new(5 , 0) , &WHITE_KING_SIDE_ROOK , board)
                },
                (Sides::Black , CastlingSide::QueenSide) => {
                    move_piece(&Square::new(2 , 7) , &BLACK_KING , board) && 
                    move_piece(&Square::new(3 , 7) , &BLACK_QUEEN_SIDE_ROOK , board)
                },
                (Sides::Black , CastlingSide::KingSide) => {
                    move_piece(&Square::new(6 , 7) , &BLACK_KING , board) && 
                    move_piece(&Square::new(5 , 7) , &BLACK_KING_SIDE_ROOK , board)
                },
                _ => todo!(),
            }
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
                promotion_piece.to_uci().to_ascii_uppercase(),
            ),
            Move::Castling { side, castling_side } => match (side , castling_side) {
                (Sides::White , CastlingSide::KingSide) => WHITE_KING_CASTLING_UCI.to_string(),
                (Sides::White , CastlingSide::QueenSide) => WHITE_QUEEN_CASTLING_UCI.to_string(),
                (Sides::Black , CastlingSide::KingSide) => BLACK_KING_CASTLING_UCI.to_string(),
                (Sides::Black , CastlingSide::QueenSide) => BLACK_QUEEN_CASTLING_UCI.to_string(),
            }
            _ => todo!(),
        }
    }
    pub fn source(&self) -> Square {
        match self {
            Move::Basic { source, .. } => *source,
            Move::Capture { source, .. } => *source,
            Move::Promotion { source, .. } => *source,
            Move::Castling { side, castling_side } => match (side , castling_side) {
                (Sides::White , CastlingSide::KingSide) => *WHITE_KING,
                (Sides::White , CastlingSide::QueenSide) => *WHITE_QUEEN_SIDE_ROOK,
                (Sides::Black , CastlingSide::KingSide) => *BLACK_KING,
                (Sides::Black , CastlingSide::QueenSide) => *BLACK_QUEEN_SIDE_ROOK,
            }
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
