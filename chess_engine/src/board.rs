use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use crate::pieces::{Sides,Pieces, PIECES_COUNT, SIDES_COUNT};

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; PIECES_COUNT]; SIDES_COUNT],
    pub sides: [Bitboard; SIDES_COUNT],
}

impl Board {
    pub fn occupied(&self) -> Bitboard {
        self.sides[Sides::White] | self.sides[Sides::Black]
    }

    pub fn empty(&self) -> Bitboard {
        !self.occupied()
    }

    pub fn enemy(&self, side: Sides) -> Bitboard {
        self.sides[side.other()]
    }

    pub fn friendly(&self, side: Sides) -> Bitboard {
        self.sides[side]
    }
    
    pub fn new() -> Self {
        let mut board = Board::default();

        board.pieces[Sides::White][Pieces::Pawn] |= Bitboard(255 << 8);
        board.pieces[Sides::Black][Pieces::Pawn] |= Bitboard(255 << (8 * 6));

        board.pieces[Sides::White][Pieces::Rook] |= Bitboard(1 | 1 << 7);
        board.pieces[Sides::Black][Pieces::Rook] |= Bitboard(1 << (8 * 7) | 1 << (8 * 7 + 7));

        board.pieces[Sides::White][Pieces::Knight] |= Bitboard(1 << 1 | 1 << 6);
        board.pieces[Sides::Black][Pieces::Knight] |= Bitboard(1 << (8 * 7 + 1) | 1 << (8 * 7 + 6));

        board.pieces[Sides::White][Pieces::Bishop] |= Bitboard(1 << 2 | 1 << 5);
        board.pieces[Sides::Black][Pieces::Bishop] |= Bitboard(1 << (8 * 7 + 2) | 1 << (8 * 7 + 5));

        board.pieces[Sides::White][Pieces::Queen] |= Bitboard(1 << 3);
        board.pieces[Sides::Black][Pieces::Queen] |= Bitboard(1 << (8 * 7 + 4));

        board.pieces[Sides::White][Pieces::King] |= Bitboard(1 << 4);
        board.pieces[Sides::Black][Pieces::King] |= Bitboard(1 << (8 * 7 + 3));

        board.sides[Sides::White] = board.pieces[Sides::White]
            .iter()
            .fold(Bitboard(0), |acc, x| acc | *x);
        board.sides[Sides::Black] = board.pieces[Sides::Black]
            .iter()
            .fold(Bitboard(0), |acc, x| acc | *x);

            board

    }

}

impl Default for Board {
    fn default() -> Self {
       Self {
            pieces: [[Bitboard(0); PIECES_COUNT]; SIDES_COUNT],
            sides: [Bitboard(0); SIDES_COUNT],
       } 
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bitboard(pub u64);

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard(0)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..8).rev() {
            // Reverse row order
            for j in 0..8 {
                write!(f, "{}", (self.0 >> (i * 8 + j)) & 1)?; // No longer 63 - (i * 8 + j)
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
