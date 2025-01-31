use crate::attacks::{ sliding_attacks, step_attacks};
use crate::board::Bitboard;
use crate::pieces::{Pieces, Sides, ALL_PIECES, PIECES_COUNT, SIDES_COUNT};

const ROOK_RAY_INDEX: [usize; 4] = [0, 1, 4, 5];
const BISHOP_RAY_INDEX: [usize; 4] = [2, 3, 6, 7];
const KNIGHT_DELTAS: [i8; 8] = [15, 17, 10, 6, -15, -17, -10, -6];
const BLACK_PAWN_DELTAS: [i8; 2] = [-7, -9];
const KING_DELTAS: [i8; 8] = [1, -1, 8, -8, 7, -7, 9, -9];
const WHITE_PAWN_DELTAS: [i8; 2] = [7, 9];

pub struct Game {
    pieces: [[Bitboard; PIECES_COUNT]; SIDES_COUNT],
    sides: [Bitboard; SIDES_COUNT],
}

impl Game {
    pub fn new() -> Self {
        Game {
            pieces: Default::default(),
            sides: Default::default(),
        }
    }

    pub fn init_board(&mut self) {
        self.pieces[Sides::White][Pieces::Pawn] |= Bitboard(255 << 8);
        self.pieces[Sides::Black][Pieces::Pawn] |= Bitboard(255 << (8 * 6));

        self.pieces[Sides::White][Pieces::Rook] |= Bitboard(1 | 1 << 7);
        self.pieces[Sides::Black][Pieces::Rook] |= Bitboard(1 << (8 * 7) | 1 << (8 * 7 + 7));

        self.pieces[Sides::White][Pieces::Knight] |= Bitboard(1 << 1 | 1 << 6);
        self.pieces[Sides::Black][Pieces::Knight] |= Bitboard(1 << (8 * 7 + 1) | 1 << (8 * 7 + 6));

        self.pieces[Sides::White][Pieces::Bishop] |= Bitboard(1 << 2 | 1 << 5);
        self.pieces[Sides::Black][Pieces::Bishop] |= Bitboard(1 << (8 * 7 + 2) | 1 << (8 * 7 + 5));

        self.pieces[Sides::White][Pieces::Queen] |= Bitboard(1 << 3);
        self.pieces[Sides::Black][Pieces::Queen] |= Bitboard(1 << (8 * 7 + 4));

        self.pieces[Sides::White][Pieces::King] |= Bitboard(1 << 4);
        self.pieces[Sides::Black][Pieces::King] |= Bitboard(1 << (8 * 7 + 3));

        self.sides[Sides::White] = self.pieces[Sides::White]
            .iter()
            .fold(Bitboard(0), |acc, x| acc | *x);
        self.sides[Sides::Black] = self.pieces[Sides::Black]
            .iter()
            .fold(Bitboard(0), |acc, x| acc | *x);
    }

    pub fn occupied(&self) -> Bitboard {
        self.sides[Sides::White] | self.sides[Sides::Black]
    }

    pub fn in_check(&self , side : Sides) {
        todo!()
    }

    pub fn legal_moves(&self , side_to_move : Sides) {
    } 

    pub fn pseudo_legal_moves(&self , side_to_move : Sides) -> Vec<Bitboard> {
        let mut moves = Vec::new();
        let occupied = self.occupied();

        for piece in ALL_PIECES {
            for i in 0..64 {
                let piece_bb = self.pieces[side_to_move][piece];
                if piece_bb.0 ^ (1 << i) == 0 {
                    continue;
                }

                let mut bb = match piece {
                    Pieces::Pawn => match side_to_move {
                        Sides::Black => step_attacks(i, &BLACK_PAWN_DELTAS),
                        Sides::White => step_attacks(i , &WHITE_PAWN_DELTAS)
                    }
                    Pieces::Rook => sliding_attacks(i , occupied , &ROOK_RAY_INDEX),
                    Pieces::Knight => step_attacks(i , &KNIGHT_DELTAS),
                    Pieces::Bishop => sliding_attacks(i, occupied , &BISHOP_RAY_INDEX),
                    Pieces::Queen => sliding_attacks(i , occupied , &ROOK_RAY_INDEX) | sliding_attacks(i , occupied , &BISHOP_RAY_INDEX),
                    Pieces::King => step_attacks(i , &KING_DELTAS),
                };

                moves.push(bb);

            }
        }

        moves
    }


}
