use crate::board::BitBoard;

#[derive(Clone)]
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

    pub fn empty(&self) -> u64 {
        !self.sides[Sides::WHITE].0 & !self.sides[Sides::BLACK].0
    }

    pub fn enemy(&self, side: usize) -> u64 {
        self.sides[match side {
            Sides::WHITE => Sides::BLACK,
            _ => Sides::WHITE,
        }]
        .0
    }

    pub fn find_piece(&self, side: usize, sqr: BitBoard) -> usize {
        for piece in Pieces::all() {
            if (sqr.0 & self.pieces[side][piece].0) > 0 {
                return piece;
            }
        }
        return Pieces::EMPTY;
    }

    pub fn legal_moves(&mut self, start: Square) {
        let init = BitBoard::from_square(start.clone());
        for side in [Sides::WHITE, Sides::BLACK] {
            if init.0 & self.sides[side].0 > 0 {
                let piece = self.find_piece(side, init);

                match piece {
                    Pieces::PAWN => {
                        let x = self.legal_pawn_moves(start, side);
                        println!("{}", x);
                    }
                    _ => {}
                }
                break;
            }
        }
    }

    pub fn legal_pawn_moves(&self, start: Square, side: usize) -> BitBoard {
        let init = BitBoard::from_square(start);
        let mut board = BitBoard(0);

        // Moving one ahead
        board.0 ^= self.empty() & (init.0 << ROW);
        // Moving 2 ahead if on starting row
        board.0 ^= self.empty() & (init.0 << (ROW * 2)) & (self.empty() & (init.0 << ROW)) << ROW;
        // Taking enemy on the right
        board.0 ^= self.enemy(side) & (init.0 << (ROW + 1));
        // Taking enemy on the left
        board.0 ^= self.enemy(side) & (init.0 << (ROW - 1));

        board
    }

    pub fn legal_rook_moves(&self, start: Square, side: usize) -> BitBoard {
        let init = BitBoard::from_square(start);
        let mut board = BitBoard(0);

        board ^= self.empty() & board
    }
}
