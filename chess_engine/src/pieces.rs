use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::board::Bitboard;

pub const SIDES_COUNT: usize = 2;
pub const PIECES_COUNT: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sides {
    White = 0,
    Black = 1,
}

pub const ALL_SIDES: [Sides; 2] = [Sides::White, Sides::Black];

impl Sides {
    pub fn other(&self) -> Sides {
        match self {
            Sides::White => Sides::Black,
            Sides::Black => Sides::White,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pieces {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

const RELATIVE_PAWN_STRENGTH: i32 = 10;
const RELATIVE_KNIGHT_STRENGTH: i32 = 30;
const RELATIVE_BISHOP_STRENGTH: i32 = 30;
const RELATIVE_ROOK_STRENGTH: i32 = 50;
const RELATIVE_QUEEN_STRENGTH: i32 = 90;
const RELATIVE_KING_STRENGTH: i32 = 900;
impl Pieces {
    pub fn relative_strength(&self) -> i32 {
        match self {
            Self::Pawn => RELATIVE_PAWN_STRENGTH,
            Self::Knight => RELATIVE_KNIGHT_STRENGTH,
            Self::Bishop => RELATIVE_BISHOP_STRENGTH,
            Self::Rook => RELATIVE_ROOK_STRENGTH,
            Self::Queen => RELATIVE_QUEEN_STRENGTH,
            Self::King => RELATIVE_KING_STRENGTH,
        }
    }
}

impl Display for Pieces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self {
            Pieces::Pawn => "Pawn",
            Pieces::Rook => "Rook",
            Pieces::Knight => "Knight",
            Pieces::Bishop => "Bishop",
            Pieces::Queen => "Queen",
            Pieces::King => "King",
        };
        write!(f, "{}", piece)
    }
}

pub const ALL_PIECES: [Pieces; 6] = [
    Pieces::Pawn,
    Pieces::Rook,
    Pieces::Knight,
    Pieces::Bishop,
    Pieces::Queen,
    Pieces::King,
];

impl Index<Sides> for [[Bitboard; PIECES_COUNT]; SIDES_COUNT] {
    type Output = [Bitboard; PIECES_COUNT];

    fn index(&self, index: Sides) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<Sides> for [Bitboard; SIDES_COUNT] {
    type Output = Bitboard;

    fn index(&self, index: Sides) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<Pieces> for [Bitboard; PIECES_COUNT] {
    type Output = Bitboard;

    fn index(&self, index: Pieces) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Sides> for [[Bitboard; PIECES_COUNT]; SIDES_COUNT] {
    fn index_mut(&mut self, index: Sides) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl IndexMut<Pieces> for [Bitboard; PIECES_COUNT] {
    fn index_mut(&mut self, index: Pieces) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl IndexMut<Sides> for [Bitboard; SIDES_COUNT] {
    fn index_mut(&mut self, index: Sides) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
