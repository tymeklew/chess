use std::cmp::max;

use crate::{board::Board, game::Game, pieces::{Sides, ALL_PIECES, ALL_SIDES}};



pub fn evaluate(board : &Board) -> i32 {
    let mut value = 0;

    for side in ALL_SIDES {
        for piece in ALL_PIECES {
            let sign = match side {
                Sides::White => 1,
                Sides::Black => -1
            };

            for i in 0..64 {
                if board.pieces[side][piece].0 & (1 << i) != 0 {
                    value += sign * piece.relative_strength();
                }
            }
        }
    } 

    value
}

// Use the minimax algorithm to determine the best move
pub fn minimax(board : &Board , mut depth : usize , maximizing : bool) -> i32 {
    if depth == 0 {
        return 0;
    } 

    let side = if maximizing { Sides::White} else {Sides::Black};

    let mut value = 0;
    for mv in board.pseudo_legal_moves(side) {
        let mut new_board = board.clone();
        mv.apply(&mut new_board);

        value += evaluate(&new_board) + minimax(&new_board, depth - 1, !maximizing);
    }

    return value;
}
