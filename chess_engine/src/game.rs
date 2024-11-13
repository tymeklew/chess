use crate::board::BitBoard;

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

    pub fn legal_moves(&mut self, start: Square) {
        let initial = BitBoard::from_square(start);

        let mut moves = BitBoard(0);
        moves.0 ^= !self.sides[Sides::BLACK].0 & (initial.0 << ROW);
        moves.0 ^= !self.sides[Sides::BLACK].0 & (initial.0 << (ROW * 2));

        moves.0 ^= self.sides[Sides::BLACK].0 & (initial.0 << (ROW - 1));
        moves.0 ^= self.sides[Sides::BLACK].0 & (initial.0 << (ROW + 1));

        println!("{}", moves);
    }
}
