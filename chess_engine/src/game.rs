use std::iter::empty;

use crate::board::BitBoard;

pub enum LegalMove {
    Move,
    Attack,
    Promotion,
    Castle,
    EnPassant,
}

#[derive(Clone, Copy)]
pub struct Square(pub u8, pub u8);
impl Square {
    pub fn new(row: u8, file: u8) -> Self {
        Self(row, file)
    }
}

pub const FULL_ROW: u64 = 2_u64.pow(8) - 1;
pub const ROW: u64 = 8;
pub const COL: u64 = 8;

pub struct Pieces {}
impl Pieces {
    pub const PAWN: usize = 0;
    pub const ROOK: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const BISHOP: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
    pub const EMPTY: usize = 6;

    pub fn all() -> Vec<usize> {
        vec![
            Self::PAWN,
            Self::ROOK,
            Self::KNIGHT,
            Self::BISHOP,
            Self::QUEEN,
            Self::KING,
        ]
    }
}

pub struct Sides {}
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

const NUM_PIECES: usize = 8;
const NUM_SIDES: usize = 2;

pub struct Game {
    pub pieces: [[BitBoard; NUM_PIECES]; NUM_SIDES],
    pub sides: [BitBoard; NUM_SIDES],
}

impl Game {
    pub fn new() -> Self {
        Self {
            pieces: [[BitBoard(0); NUM_PIECES]; NUM_SIDES],
            sides: [BitBoard(0); NUM_SIDES],
        }
    }

    /// Initialize default chess board
    pub fn init(&mut self) {
        self.pieces[Sides::WHITE][Pieces::PAWN].0 ^= FULL_ROW << ROW;
        self.pieces[Sides::BLACK][Pieces::PAWN].0 ^= FULL_ROW << ((ROW - 2) * 8);

        for i in 0..3 {
            self.pieces[Sides::WHITE][1 + i].0 ^= 2_u64.pow(i as u32) + 2_u64.pow(7 - i as u32);
            self.pieces[Sides::BLACK][1 + i].0 ^=
                (2_u64.pow(i as u32) + 2_u64.pow(7 - i as u32)) << (7 * ROW);
        }

        self.pieces[Sides::WHITE][Pieces::QUEEN].0 ^= 2_u64.pow(7 - 4);
        self.pieces[Sides::BLACK][Pieces::QUEEN].0 ^= 2_u64.pow(7 - 4) << (7 * ROW);

        self.pieces[Sides::WHITE][Pieces::KING].0 ^= 2_u64.pow(7 - 3);
        self.pieces[Sides::BLACK][Pieces::KING].0 ^= 2_u64.pow(7 - 3) << (7 * ROW);

        for i in 0..2 {
            for j in 0..6 {
                self.sides[i].0 ^= self.pieces[i][j].0;
            }
        }
    }

    /// Returns bitboard with 1's in empty squares
    pub fn empty(&self) -> BitBoard {
        BitBoard(!self.sides[Sides::WHITE].0 & !self.sides[Sides::BLACK].0)
    }

    /// Returns bitboard with 1's in place of enemy pieces
    ///* `side` - The players side
    pub fn enemy(&self, side: usize) -> BitBoard {
        BitBoard(
            self.sides[match side {
                Sides::WHITE => Sides::BLACK,
                _ => Sides::WHITE,
            }]
            .0,
        )
    }

    /// Find what type of piece is in a certain square
    /// * `sqr` - Square you are trying to find what piece is on
    pub fn find_piece(&self, sqr: Square) -> usize {
        let side = self.find_side(sqr);

        for piece in [
            Pieces::PAWN,
            Pieces::ROOK,
            Pieces::BISHOP,
            Pieces::KNIGHT,
            Pieces::QUEEN,
            Pieces::KING,
        ] {
            if (self.pieces[side][piece] & BitBoard::from_square(sqr)).0 > 0 {
                return piece;
            }
        }

        return Pieces::EMPTY;
    }

    /// Find what side the piece on the square is
    /// * `sqr` - Square you are trying to find what side it is on
    pub fn find_side(&self, sqr: Square) -> usize {
        let board = BitBoard::from_square(sqr);
        for side in [Sides::WHITE, Sides::BLACK] {
            if (board & self.sides[side]).0 > 0 {
                return side;
            }
        }
        return Sides::WHITE;
    }

    /// Generate legal moves given a square
    /// * `sqr` - Square from which to generate the legal moves
    pub fn legal_moves(&mut self, sqr: Square) -> Vec<LegalMove> {
        let side = self.find_side(sqr);
        let piece = self.find_piece(sqr);

        match piece {
            Pieces::PAWN => self.legal_pawn_moves(sqr, side),
            Pieces::ROOK => self.legal_rook_moves(sqr, side),
            _ => todo!(),
        }
    }

    /// Generate legal moves for a pawn given it's square and side
    ///* `sqr` - The square on which the pawn is
    ///* `side` - What side the pawn is on
    pub fn legal_pawn_moves(&self, sqr: Square, side: usize) -> Vec<LegalMove> {
        let init = BitBoard::from_square(sqr);
        let positive = if side == Sides::WHITE { true } else { false };
        let mvs = vec![];

        // Single move up
        if (self.empty() & init.shift(ROW, positive)).0 > 0 {}
        // Add move to LegalMoves

        let capture_mask: BitBoard = init.shift(ROW + 1, positive) ^ init.shift(ROW - 1, positive);

        mvs
    }

    /// Generate legal moves for a rook given it's square and side
    ///* `sqr` - The square on which the rook is
    ///* `side` - What side the rook is on
    pub fn legal_rook_moves(&self, sqr: Square, side: usize) -> Vec<LegalMove> {
        vec![]
    }
}
