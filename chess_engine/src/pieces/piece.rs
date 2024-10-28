use crate::{
    board::Board,
    game::{Move, Position},
};

#[derive(Debug, Clone, Copy)]
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
            PieceType::Pawn => valid_pawn_moves(pos, board, self.has_moved),
            _ => vec![],
        }
    }

    pub fn is_legal_move(&self, mov: Move) -> bool {
        todo!()
    }
}

fn valid_pawn_moves(pos: Position, board: &Board, has_moved: bool) -> Vec<Move> {
    let mut moves = Vec::new();

    if pos.row <= 5 {
        moves.push(Move::Basic(
            pos.clone(),
            Position::new(pos.row + 1, pos.col),
        ));
        if pos.row <= 6 && !has_moved {
            moves.push(Move::Basic(
                pos.clone(),
                Position::new(pos.row + 2, pos.col),
            ));
        }
    }

    moves
}
