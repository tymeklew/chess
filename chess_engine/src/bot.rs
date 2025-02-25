use std::i32;

use crate::{
    board::Board,
    moves::Move,
    pieces::{Pieces, Sides},
};

static mut COUNT : usize = 0;

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
        + 3 * (board.count_piece(Sides::White, Pieces::Knight)
        - board.count_piece(Sides::Black, Pieces::Knight))
        + (board.count_piece(Sides::White, Pieces::Bishop))
        - board.count_piece(Sides::Black, Pieces::Bishop)))
        + 1 * (board.count_piece(Sides::White, Pieces::Pawn)
        - board.count_piece(Sides::Black, Pieces::Pawn))
}

//TODO
/*pub fn merge_sort(mut moves : Vec<(i32 , Box<dyn Move>)>) -> Vec<(i32 , Box<dyn Move>)> {
    if moves.len() > 1 {
        let mid = moves.len() / 2;

        let mut left = moves;
        let right = left.split_off(mid);

        let left = merge_sort(left);
        let right = merge_sort(right);

        let mut i = 0;
        let mut j = 0;

        let mut new = Vec::new();
        while i < left.len() & j < right.len() {
            if left[i] > right[j] {
                new.push(left[i]);
                i += 1;
            }else {
                new.push(right[j]);
                j += 1;
            }
        }
        return new;
    return moves;
}*/
// Merge sort
/*pub fn sort_moves(board : &Board , side : Sides , moves : Vec<Box<dyn Move>>) -> Vec<Box<dyn Move>> {
    let mut new = board.clone();
    let values : Vec<(i32 , Box<dyn Move>)> = moves.into_iter().map(|f| {
        f.apply(&mut new);
        let score = evaluate(&new, side);
        f.undo(&mut new);
        (score , f)
    }).collect();

    merge_sort(values);


    moves
}*/

pub fn bot_move(board: &Board, depth: usize, side: Sides) -> (i32, Option<Move>) {
    let mut board = board.clone();
    let x = match side {
        Sides::White => maxi(&mut board, depth, side, i32::MIN, i32::MAX),
        Sides::Black => mini(&mut board, depth, side, i32::MIN, i32::MAX),
    };

    unsafe  {
        println!("Count : {}" , COUNT);
    }

    x
}

pub fn maxi(
    board: &mut Board,
    depth: usize,
    side: Sides,
    mut alpha: i32,
    beta: i32,
) -> (i32, Option<Move>) {
    if depth == 0 {
        return (evaluate(board, side), None);
    };

    let mut max = i32::MIN;
    let mut best_move = None;
    for mv in board.legal_moves(side) {
        println!("depth : {} , move : {}" , depth , mv);
        unsafe  {
            COUNT += 1;
        }
        if !mv.apply(board) {
            continue;
        }
        let (score, _) = mini(board, depth - 1, side.other(), alpha, beta);
        mv.undo(board);

        if score > max {
            max = score;
            best_move = Some(mv);
        }

        /*alpha = alpha.max(score);
        if beta <= alpha {
            break;
        }*/
    }

    return (max, best_move);
}
pub fn mini(
    board: &mut Board,
    depth: usize,
    side: Sides,
    alpha: i32,
    mut beta: i32,
) -> (i32, Option<Move>) {
    if depth == 0 {
        println!("Reached Depth 0");
        return (evaluate(board, side), None);
    };

    let mut min = i32::MAX;
    let mut best_move = None;
    println!("depth {}: {} moves", depth, board.legal_moves(side).len());
    for mv in board.legal_moves(side) {
        //println!("depth : {} , move : {}" , depth , mv);
        unsafe  {
            COUNT += 1;
        }
        //println!("Before move {} (depth {}):\n", mv, depth);
        //board.display();
        if !mv.apply(board) {
            println!("Faiked : {}" , mv);
            continue;
        }
        //println!("After move {} (depth {}):\n", mv, depth);
        //board.display();
        let (score, _) = maxi(board, depth - 1, side.other(), alpha, beta);
        mv.undo(board);
        //println!("After undo {} (depth {}):\n", mv, depth);
        //board.display();

        if score < min {
            min = score;
            best_move = Some(mv);
        }

        /*beta = beta.min(score);
        if beta <= alpha {
            break;
        }*/
    }

    return (min, best_move);
}
