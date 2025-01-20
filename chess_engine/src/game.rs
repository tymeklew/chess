use crate::{board::BitBoard, state::State};

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
    pub fn new(file: u8, rank: u8) -> Self {
        Self(file, rank)
    }
}

impl std::ops::Add for Square {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0 , self.1 + rhs.1)
    }
}

pub const FULL_ROW: u64 = 2_u64.pow(8) - 1;
pub const FULL_COL : u64 = 0x0101010101010101; 
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
    pub turn : usize,
    pub pieces: [[BitBoard; NUM_PIECES]; NUM_SIDES],
    pub sides: [BitBoard; NUM_SIDES],
    state : State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            pieces: [[BitBoard(0); NUM_PIECES]; NUM_SIDES],
            sides: [BitBoard(0); NUM_SIDES],
            state : State::new(),
            turn : 0,
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

    fn find_piece_type(&self,  sqr : Square) -> Option<usize> {
        let bb = BitBoard::from_square(sqr);

        if (bb.0 & (self.sides[Sides::WHITE].0 | self.sides[Sides::BLACK].0)) == 0 {
            return None;
        }

        Pieces::all().iter().position(|&piece| { 
            (bb.0 & (self.pieces[Sides::WHITE][piece] | self.pieces[Sides::BLACK][piece]).0) > 0
         })
    }

    fn find_side(&self , sqr : Square) -> Option<usize> {
        let bb = BitBoard::from_square(sqr);

        for i in [Sides::WHITE , Sides::BLACK] {
            if (self.sides[i].0 & bb.0) > 0 {
                return Some(i);
            }
        }
    
        None        
    }

    pub fn legal_moves(&self,  sqr : Square) -> Option<()> {
        let piece_type = self.find_piece_type(sqr)?;
        let side = self.find_side(sqr)?;

        match piece_type {
            Pieces::ROOK => self.legal_rook_moves(Square::new(5,5) , side),
            _ => {},
        }

        Some(())
    }

    // Generate a bitboard with all friendly pieces
    fn friendly(&self , side : usize) -> BitBoard{
        let mut bb = BitBoard(0);

        for i in Pieces::all() {
            bb |= self.pieces[side][i];
        } 

        bb
    } 

    // Generate a bitboard with the enemy pieces
    fn enemy(&self , side : usize) -> BitBoard {
        let mut bb = BitBoard(0);
        let opposing = if side == Sides::WHITE { Sides::BLACK } else { Sides::WHITE};

        for i in Pieces::all() {
            bb |= self.pieces[opposing][i];
        } 

        bb
    }

    pub fn legal_rook_moves(&self , sqr : Square , side : usize) {
        let mut bb = BitBoard(0);
        let occupied = self.friendly(side) | self.enemy(side);

        bb.0 |= FULL_ROW << (2_u32.pow(sqr.0 as u32));
        bb.0 |= FULL_COL << sqr.1;

        // Directions rook can travel alternate 
        let directions : [(i8 , i8) ; 4] = [
            (1 , 0),
            (0 ,1 ),
            (-1 , 0),
            (0 , -1)
        ];

        for (dx , dy) in directions {
            let x = sqr.0;
            let y = sqr.1;

            loop {

                
            }
        }





        println!("{}" , bb);
    }
}
