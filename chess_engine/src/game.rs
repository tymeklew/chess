use std::collections::HashMap;

use crate::attacks::{pawn_moves, sliding_attacks, step_attacks};
use crate::board::{Bitboard, Board};
use crate::moves::{BasicMove, Capture, Move};
use crate::pieces::{Pieces, Sides, ALL_PIECES, PIECES_COUNT, SIDES_COUNT};
use crate::square::Square;


const ROOK_RAY_INDEX: [usize; 4] = [0, 1, 4, 5];
const BISHOP_RAY_INDEX: [usize; 4] = [2, 3, 6, 7];
const KNIGHT_DELTAS: [i8; 8] = [15, 17, 10, 6, -15, -17, -10, -6];
const BLACK_PAWN_DELTAS: [i8; 2] = [-7, -9];
const KING_DELTAS: [i8; 8] = [1, -1, 8, -8, 7, -7, 9, -9];
const WHITE_PAWN_DELTAS: [i8; 2] = [7, 9];

pub struct Game {
    turn: usize,
    board : Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board : Board::default(),
            turn: 0,
        }
    }

    pub fn mv(&mut self , m : Box<dyn Move>) {
        m.apply(&mut self.board);
        self.turn += 1;
    }

    pub fn king_in_check(&mut self , side : Sides) -> bool {
        let mvs = self.pseudo_legal_moves(side.other());

        for mv in mvs {
            // Check if the move is a capture
            if let Some(Pieces::King) = mv.capture() {
                return true;
            }
        }

        false
    }

    pub fn legal_moves(&self, side_to_move: Sides) -> Vec<Box<dyn Move>> {
        let pseudo_legal = self.pseudo_legal_moves(side_to_move);
    
        pseudo_legal.iter().filter_map(|f| {
            let mut board = self.board.clone();
            f.apply(&mut board);
            if !self.king_in_check(side_to_move) {
                // Return a clone of the Box<dyn Move> if it's a valid move
                Some(f.clone())  // or Box::new(f.clone()) depending on your exact implementation
            } else {
                None
            }
        }).collect()
    }


    // Corrolates the position of the pieces on the board to the bitboard for attack
    pub fn pseudo_legal_moves(&self, side_to_move: Sides) -> Vec<Box<dyn Move>> {
        let occupied = self.board.occupied();
        let mut moves : Vec<Box<dyn Move>> = Vec::new();

        for piece in ALL_PIECES {
            for i in 0..64 {
                let piece_bb = self.board.pieces[side_to_move][piece];
                if piece_bb.0 & (1 << i) == 0 {
                    continue;
                }

                let bb = match piece {
                    Pieces::Pawn => {
                        let bb = match side_to_move {
                            Sides::Black => step_attacks(i, &BLACK_PAWN_DELTAS),
                            Sides::White => step_attacks(i, &WHITE_PAWN_DELTAS),
                        };
                        bb & self.board.enemy(side_to_move) | pawn_moves(i, side_to_move, occupied)
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

                let basic_moves = bb & !self.board.friendly(side_to_move);
                let captures = bb & self.board.enemy(side_to_move);

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
                        let captured_piece = ALL_PIECES.iter().find(|&&x| {
                            self.board.pieces[side_to_move.other()][x].0 & (1 << j) != 0
                        }).unwrap();
                        moves.push(Box::new(Capture::new(
                            Square::from_idx(i),
                            Square::from_idx(j),
                            *captured_piece,
                        )));
                    }
                }
            }
        }
        moves
    }
}
