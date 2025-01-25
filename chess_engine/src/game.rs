use core::panic;

use crate::{board::BitBoard, state::State};

pub enum LegalMove {
    Move,
    Attack,
    Promotion, // engine should always promote to knight TODO
    Castle,
    EnPassant, // not real move
}

#[derive(Clone, Copy)]
pub struct Square(pub u8, pub u8);
impl Square {
    pub fn new(file: u8, rank: u8) -> Self {
        Self(file, rank)
    }
}

impl std::ops::Add for Square {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub const FULL_ROW: u64 = 2_u64.pow(8) - 1;
pub const FULL_COL: u64 = 0x0101010101010101;
pub const ROW: u64 = 8;
pub const COL: u64 = 8;

const ROOK_MOVEMENT: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const BISHOP_MOVEMENT: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
// Only contains right side as will be mirrored to the other side
const RIGHT_SIDE_KNIGHT_MOVEMENT: [(i8, i8); 4] = [(1, 2), (2, 1), (2, -1), (1, -2)];
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
    pub turn: usize,
    pub pieces: [[BitBoard; NUM_PIECES]; NUM_SIDES],
    pub sides: [BitBoard; NUM_SIDES],
    state: State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            pieces: [[BitBoard(0); NUM_PIECES]; NUM_SIDES],
            sides: [BitBoard(0); NUM_SIDES],
            state: State::new(),
            turn: 0,
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

    fn find_piece_type(&self, sqr: Square) -> Option<usize> {
        let bb = BitBoard::from_square(sqr);

        if (bb.0 & (self.sides[Sides::WHITE].0 | self.sides[Sides::BLACK].0)) == 0 {
            return None;
        }

        Pieces::all().iter().position(|&piece| {
            (bb.0 & (self.pieces[Sides::WHITE][piece] | self.pieces[Sides::BLACK][piece]).0) > 0
        })
    }

    fn find_side(&self, sqr: Square) -> Option<usize> {
        let bb = BitBoard::from_square(sqr);

        for i in [Sides::WHITE, Sides::BLACK] {
            if (self.sides[i].0 & bb.0) > 0 {
                return Some(i);
            }
        }

        None
    }

    pub fn legal_moves(&self, sqr: Square) -> Option<()> {
        let piece_type = self.find_piece_type(sqr)?;
        let side = self.find_side(sqr)?;

        let bb = match piece_type {
            Pieces::PAWN => self.legal_pawn_moves(sqr, side),
            Pieces::BISHOP => self.legal_bishop_moves(sqr, side),
            Pieces::KNIGHT => self.legal_knight_moves(Square::new(2, 3), side),
            Pieces::ROOK => self.legal_rook_moves(sqr, side),
            Pieces::QUEEN => self.legal_queen_moves(sqr, side),
            _ => todo!(),
        };

        println!("{}", bb);
        Some(())
    }

    // Generate a bitboard with all friendly pieces
    fn friendly(&self, side: usize) -> BitBoard {
        let mut bb = BitBoard(0);

        for i in Pieces::all() {
            bb |= self.pieces[side][i];
        }

        bb
    }

    // Generate a bitboard with the enemy pieces
    fn enemy(&self, side: usize) -> BitBoard {
        let mut bb = BitBoard(0);
        let opposing = if side == Sides::WHITE {
            Sides::BLACK
        } else {
            Sides::WHITE
        };

        for i in Pieces::all() {
            bb |= self.pieces[opposing][i];
        }

        bb
    }

    pub fn occupied(&self, side: usize) -> BitBoard {
        self.enemy(side) | self.friendly(side)
    }

    pub fn legal_pawn_moves(&self, sqr: Square, side: usize) -> BitBoard {
        let mut bb = BitBoard(0);
        let direction = if side == Sides::WHITE { 1 } else { -1 };
        let occupied = self.friendly(side) | self.enemy(side);

        // Check initial double movement
        let moved = match side {
            Sides::WHITE => sqr.1 != 1,
            Sides::BLACK => sqr.1 != 6,
            _ => panic!("Side that doesnt exist"),
        };

        let single = BitBoard::from_square(Square::new(sqr.0, sqr.1 + 1));
        let double = BitBoard::from_square(Square::new(sqr.0, sqr.1 + 2));

        // Standard Movement
        if !moved && ((single | double) & occupied).0 == 0 {
            bb |= single | double;
        } else if (single & occupied).0 == 0 {
            bb |= single;
        }

        let y = match sqr.1.checked_add_signed(direction) {
            Some(n) if n <= 7 => n,
            _ => panic!("Not bad"),
        };

        // Attacking
        for i in [1, -1] {
            let x = match sqr.0.checked_add_signed(i) {
                Some(n) if n <= 7 => n,
                _ => continue,
            };

            let mask = BitBoard::from_square(Square::new(x, y));
            bb |= mask & self.enemy(side);
        }
        bb
    }

    pub fn legal_bishop_moves(&self, sqr: Square, side: usize) -> BitBoard {
        self.generate_sliding_moves(sqr, side, &BISHOP_MOVEMENT)
    }

    pub fn legal_knight_moves(&self, sqr: Square, side: usize) -> BitBoard {
        let mut bb = BitBoard(0);
        for (dx, dy) in RIGHT_SIDE_KNIGHT_MOVEMENT {
            let x = match sqr.0.checked_add_signed(dx) {
                Some(n) if n <= 7 => n,
                _ => continue,
            };
            let y = match sqr.1.checked_add_signed(dy) {
                Some(n) if n <= 7 => n,
                _ => continue,
            };

            let mv = BitBoard::from_square(Square::new(x, y));
            bb |= mv;
        }
        let mirror = bb.mirror_h();

        bb |= mirror;
        bb &= !self.friendly(side);

        bb
    }

    pub fn legal_rook_moves(&self, sqr: Square, side: usize) -> BitBoard {
        self.generate_sliding_moves(sqr, side, &ROOK_MOVEMENT)
    }

    pub fn legal_queen_moves(&self, sqr: Square, side: usize) -> BitBoard {
        self.generate_sliding_moves(sqr, side, &BISHOP_MOVEMENT)
            | self.generate_sliding_moves(sqr, side, &ROOK_MOVEMENT)
    }

    pub fn legal_king_moves(&self, sqr: Square, side: usize) -> BitBoard {
        let bb = BitBoard(0);

        bb
    }

    fn generate_sliding_moves(
        &self,
        sqr: Square,
        side: usize,
        directions: &[(i8, i8)],
    ) -> BitBoard {
        let occupied = self.occupied(side);
        let mut bb = BitBoard(0);

        for (dx, dy) in directions {
            let mut x = sqr.0;
            let mut y = sqr.1;

            loop {
                x = match x.checked_add_signed(*dx) {
                    Some(n) if n <= 7 => n,
                    _ => break,
                };

                y = match y.checked_add_signed(*dy) {
                    Some(n) if n <= 7 => n,
                    _ => break,
                };

                let mv = BitBoard::from_square(Square::new(x, y));
                if (mv & self.enemy(side)).0 > 0 {
                    bb |= mv;
                    break;
                } else if (mv & occupied).0 > 0 {
                    break;
                }
                bb |= mv;
            }
        }
        bb
    }
}
