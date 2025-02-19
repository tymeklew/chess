use std::{i32, process::ExitStatus};

use crate::{
    board::Board,
    moves::Move,
    pieces::{Pieces, Sides, ALL_PIECES, ALL_SIDES},
};

pub fn evaluate(board: &Board, side: Sides) -> i32 {
    let sign = match side {
        Sides::White => 1,
        Sides::Black => -1,
    };

    // Claude Shannon's evaluation function
    sign * (200
        * (board.count_piece(Sides::White, Pieces::King)
            - board.count_piece(Sides::Black, Pieces::King))
        + 9 * (board.count_piece(Sides::White, Pieces::Queen)
            - board.count_piece(Sides::Black, Pieces::Queen))
        + 5 * (board.count_piece(Sides::White, Pieces::Rook)
            - board.count_piece(Sides::Black, Pieces::Rook)
            + board.count_piece(Sides::White, Pieces::Knight)
            - board.count_piece(Sides::Black, Pieces::Knight))
        + 3 * (board.count_piece(Sides::White, Pieces::Bishop)
            - board.count_piece(Sides::Black, Pieces::Bishop)))
}

pub fn bot_move(board: &Board, depth: usize, side: Sides) -> (i32, Option<Box<dyn Move>>) {
    match side {
        Sides::White => maxi(board, depth, side, i32::MIN, i32::MAX),
        Sides::Black => mini(board, depth, side, i32::MIN, i32::MAX),
    }
}

pub fn maxi(
    board: &Board,
    depth: usize,
    side: Sides,
    mut alpha: i32,
    beta: i32,
) -> (i32, Option<Box<dyn Move>>) {
    if depth == 0 {
        return (evaluate(board, side), None);
    };

    let mut max = i32::MIN;
    let mut best_move = None;
    for mv in board.legal_moves(side) {
        let mut new = board.clone();
        mv.apply(&mut new);
        let (score, _) = mini(&new, depth - 1, side.other(), alpha, beta);

        if score > max {
            max = score;
            best_move = Some(mv);
        }

        alpha = alpha.max(score);
        if beta <= alpha {
            break;
        }
    }

    return (max, best_move);
}
pub fn mini(
    board: &Board,
    depth: usize,
    side: Sides,
    alpha: i32,
    mut beta: i32,
) -> (i32, Option<Box<dyn Move>>) {
    if depth == 0 {
        return (evaluate(board, side), None);
    };

    let mut min = i32::MAX;
    let mut best_move = None;
    for mv in board.legal_moves(side) {
        let mut new = board.clone();
        mv.apply(&mut new);
        let (score, _) = maxi(&new, depth - 1, side.other(), alpha, beta);

        if score < min {
            min = score;
            best_move = Some(mv);
        }

        beta = beta.min(score);
        if beta <= alpha {
            break;
        }
    }

    return (min, best_move);
}
