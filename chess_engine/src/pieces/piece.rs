use crate::{
    board::Board,
    game::{Move, Position},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    colour: Colour,
    piece_type: PieceType,
    has_moved: bool,
}

impl Piece {
    pub fn new(colour: Colour, piece_type: PieceType) -> Self {
        Self {
            colour,
            piece_type,
            has_moved: false,
        }
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn colour(&self) -> Colour {
        self.colour
    }
    pub fn value(&self) -> i8 {
        match self.piece_type {
            PieceType::Pawn => 1,
            PieceType::Knight | PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 0,
        }
    }

    pub fn available_moves(&self, pos: Position, board: &Board) -> Vec<Move> {
        match self.piece_type {
            PieceType::Pawn => valid_pawn_moves(pos, board, self.has_moved, self.colour),
            _ => vec![],
        }
    }

    pub fn moved(&mut self) {
        self.has_moved = true;
    }

    pub fn is_legal_move(&self, mov: Move) -> bool {
        todo!()
    }
}

const BOUNDS: (i8, i8) = (0, 7);
fn bounded_add(n1: i8, n2: i8) -> Option<i8> {
    if (n1 + n2) >= 0 && (n1 + n2) <= 7 {
        return Some(n1 + n2);
    }
    None
}

fn valid_pawn_moves(pos: Position, board: &Board, has_moved: bool, colour: Colour) -> Vec<Move> {
    let mut moves = Vec::new();

    let (direction, offsets) = match colour {
        Colour::Black => (-1, [(-1, -1), (1, -1)]),
        Colour::White => (1, [(-1, 1), (1, 1)]),
    };

    if !has_moved
        && board
            .get(&Position::new(pos.col, pos.row + direction))
            .is_none()
        && board
            .get(&Position::new(pos.col, pos.row + 2 * direction))
            .is_none()
    {
        moves.push(Move::Basic(
            pos.clone(),
            Position::new(pos.col, pos.row + 2 * direction),
        ));
    }

    if board
        .get(&Position::new(pos.col, pos.row + direction))
        .is_none()
    {
        moves.push(Move::Basic(
            pos.clone(),
            Position::new(pos.col, pos.row + direction),
        ))
    }

    for (c_offset, r_offset) in offsets {
        match (
            bounded_add(pos.col, c_offset),
            bounded_add(pos.row, r_offset),
        ) {
            (Some(col), Some(row)) => match board.get(&Position::new(col, row)) {
                Some(piece) if piece.colour() != colour => {
                    moves.push(Move::Basic(pos.clone(), Position::new(col, row)))
                }
                _ => continue,
            },

            _ => {}
        }
    }

    moves
}
