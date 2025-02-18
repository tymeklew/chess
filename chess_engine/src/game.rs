use std::collections::HashMap;

use crate::attacks::{pawn_moves, sliding_attacks, step_attacks};
use crate::board::{Bitboard, Board};
use crate::moves::{BasicMove, Capture, Move};
use crate::pieces::{Pieces, Sides, ALL_PIECES, PIECES_COUNT, SIDES_COUNT};
use crate::square::Square;

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

    // Corrolates the position of the pieces on the board to the bitboard for attack
   
}
