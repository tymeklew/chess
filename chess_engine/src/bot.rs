use std::{cmp::max, num::ParseIntError};

use crate::{
    board::Board,
    game::Game,
    moves::Move,
    pieces::{Sides, ALL_PIECES, ALL_SIDES},
    tree::Node,
};

pub fn evaluate(board: &Board) -> i32 {
    let mut value = 0;

    for side in ALL_SIDES {
        for piece in ALL_PIECES {
            let sign = match side {
                Sides::White => 1,
                Sides::Black => -1,
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
pub fn generate_tree(board: &Board, depth: usize, max_depth: usize, side: Sides) -> Vec<Box<Node>> {
    let mvs = board.pseudo_legal_moves(side);

    let mut stuff = Vec::new();
    for mv in mvs {
        //println!("{}" , mv);
        let mut new = board.clone();
        mv.apply(&mut new);

        let mut parent = Node::new(evaluate(&new), Some(mv));
        if depth != max_depth {
            let children = generate_tree(&new, depth + 1, max_depth, side.other());
            parent.add(children);
        }
        stuff.push(Box::new(parent));
    }

    return stuff;
}
