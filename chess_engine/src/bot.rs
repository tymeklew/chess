use std::{cmp::max, i32, num::ParseIntError};

use crate::{
    board::Board,
    game::Game,
    moves::Move,
    pieces::{Sides, ALL_PIECES, ALL_SIDES},
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

pub fn mv(board : &Board , depth : usize , side : Sides) -> (i32 , Option<Box<dyn Move>>) {
    match side {
        Sides::White => maxi(board, depth, side),
        Sides::Black => mini(board, depth, side)
    }
} 

pub fn maxi(board: &Board , depth : usize , side : Sides) -> (i32 , Option<Box<dyn Move>>) {
    if depth == 0 { return (evaluate(board) , None) };

    let mut max = i32::MIN;
    let mut yap = None;
    for mv in board.pseudo_legal_moves(side) {
        let mut new = board.clone();
        mv.apply(&mut new);
        let (score , _) = mini(&new, depth - 1, side.other());


        if score > max {
            max = score;
            yap = Some(mv);
        }
    }

    return (max , yap);
}
pub fn mini(board: &Board , depth : usize , side : Sides) -> (i32 , Option<Box<dyn Move>>) {
    if depth == 0 { return (evaluate(board) , None) };

    let mut min = i32::MAX;
    let mut yap = None;
    for mv in board.pseudo_legal_moves(side) {
        let mut new = board.clone();
        mv.apply(&mut new);
        let (score , _) = maxi(&new, depth - 1, side.other());

        if score < min {
            min = score;
            yap = Some(mv);
        }
    }

    return (min , yap);
}