use crate::attacks::{pawn_moves, sliding_attacks, step_attacks};
use crate::moves::{BasicMove, Capture, Move};
use crate::pieces::{self, Pieces, Sides, ALL_PIECES, ALL_SIDES, PIECES_COUNT, SIDES_COUNT};
use crate::square::Square;
use core::panic;
use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; PIECES_COUNT]; SIDES_COUNT],
    pub sides: [Bitboard; SIDES_COUNT],
}

impl Board {
    pub fn occupied(&self) -> Bitboard {
        self.sides[Sides::White] | self.sides[Sides::Black]
    }
    pub fn display(&self) {
        let occupied = self.occupied();
        for i in (0..8).rev() {
           for j in 0..8 {
                let idx = i * 8 + j;
                if Bitboard(1 << idx) & occupied != Bitboard(0) {
                    print!("{}" , match self.get_side(Square::from_idx(idx)) {
                        Sides::Black => "B",
                        Sides::White => "W"
                    });
                    continue;
                }
                print!(" ");
           } 
            println!(); 
        }
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

    pub fn get_piece(&self, pos: Square) -> Pieces {
        let side = self.get_side(pos);

        for piece in ALL_PIECES {
            if self.pieces[side][piece].0 & (1 << pos.idx()) != 0 {
                return piece;
            }
        }

        return Pieces::Pawn;
    }

    pub fn get_side(&self, pos: Square) -> Sides {
        let idx = pos.idx();

        for side in ALL_SIDES {
            if self.sides[side].0 & (1 << idx) != 0 {
                return side;
            }
        }

        // Need to fix
        Sides::White
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

    // Generates moves including pawn moves
    pub fn pseudo_legal_moves(&self, side_to_move: Sides) -> Vec<Box<dyn Move>> {
        const ROOK_RAY_INDEX: [usize; 4] = [0, 1, 4, 5];
        const BISHOP_RAY_INDEX: [usize; 4] = [2, 3, 6, 7];
        const KNIGHT_DELTAS: [i8; 8] = [15, 17, 10, 6, -15, -17, -10, -6];
        const KING_DELTAS: [i8; 8] = [1, -1, 8, -8, 7, -7, 9, -9];
        const WHITE_PAWN_DELTAS: [i8; 2] = [7, 9];
        const BLACK_PAWN_DELTAS: [i8; 2] = [-7, -9];

        let occupied = self.occupied();
        let mut moves: Vec<Box<dyn Move>> = Vec::new();

        for piece in ALL_PIECES {
            for i in 0..64 {
                let piece_bb = self.pieces[side_to_move][piece];
                if piece_bb.0 & (1 << i) == 0 {
                    continue;
                }

                let bb = match piece {
                    Pieces::Pawn => {
                        let bb = match side_to_move {
                            Sides::Black => step_attacks(i, &BLACK_PAWN_DELTAS),
                            Sides::White => step_attacks(i, &WHITE_PAWN_DELTAS),
                        };
                        bb & self.enemy(side_to_move) | (pawn_moves(i, side_to_move, occupied) & !self.occupied())
                    }
                    Pieces::Rook => sliding_attacks(i, occupied, &ROOK_RAY_INDEX),
                    Pieces::Knight => step_attacks(i, &KNIGHT_DELTAS),
                    Pieces::Bishop => sliding_attacks(i, occupied, &BISHOP_RAY_INDEX),
                    Pieces::Queen => {
                        sliding_attacks(i, occupied, &ROOK_RAY_INDEX)
                            | sliding_attacks(i, occupied, &BISHOP_RAY_INDEX)
                    }
                    Pieces::King => step_attacks(i, &KING_DELTAS),
                };

                if bb.0 == 0 {
                    continue;
                }

                let basic_moves = bb & !self.friendly(side_to_move);
                let captures = bb & self.enemy(side_to_move);

                for j in 0..64 {
                    if basic_moves.0 & (1 << j) != 0 {
                        moves.push(Box::new(BasicMove::new(
                            Square::from_idx(i),
                            Square::from_idx(j),
                        )));
                    }
                }

                for j in 0..64 {
                    if captures.0 & (1 << j) != 0 {
                        let captured_piece = self.get_piece(Square::from_idx(j));
                        moves.push(Box::new(Capture::new(
                            Square::from_idx(i),
                            Square::from_idx(j),
                            captured_piece,
                        )));
                    }
                }
            }
        }
        moves
    }

    pub fn from_fen(input : String) -> Board {
        let mut board = Board::default();
        let parts  : Vec<&str> = input.split(" ").collect();

        // Handle placment
        let placement = parts.get(0).unwrap();
        for (i , row) in placement.split("/").enumerate() {
            let mut index = (7 - i) * 8;
            for chr in row.chars() {
                if chr.is_numeric() {
                    index += chr.to_digit(10).unwrap() as usize;
                    continue;
                }

                let side = if chr.is_uppercase() {  Sides::White } else { Sides::Black };
                let piece = match chr.to_ascii_lowercase() {
                    'p' => Pieces::Pawn,
                    'r' => Pieces::Rook,
                    'n' => Pieces::Knight,
                    'b' => Pieces::Bishop,
                    'q' => Pieces::Queen,
                    'k' => Pieces::King,
                    _ => panic!("Yappadoodledo")
                };

                board.sides[side] ^= Bitboard(1 << index);
                board.pieces[side][piece] ^= Bitboard(1 << index);
                index += 1;
            }
        }


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
