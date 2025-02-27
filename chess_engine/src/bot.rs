use std::i32;

use crate::{
    board::Board,
    heuristics::PAWN_TABLE,
    moves::Move,
    pieces::{Pieces, Sides},
};

static mut COUNT: usize = 0;

pub fn evaluate(board: &Board, side: Sides) -> i32 {
    let sign = match side {
        Sides::White => 1,
        Sides::Black => -1,
    };

    if board.is_checkmate(side.other()) {
        return sign * i32::MAX;
    }
    // Pawn Structure
    let mut pawn_structure = 0;
    let mut temp = board.pieces[side][Pieces::Pawn];
    while temp.0 != 0 {
        let index = temp.0.trailing_zeros() as usize;

        pawn_structure += match side {
            Sides::White => PAWN_TABLE[index / 8][index % 8],
            Sides::Black => PAWN_TABLE[7 - index / 8][index % 8],
        };
        temp.0 &= temp.0 - 1;
    }

    // Claude Shannon's evaluation function
    let material = sign
        * (5000
            * (board.count_piece(Sides::White, Pieces::King)
                - board.count_piece(Sides::Black, Pieces::King))
            + 900
                * (board.count_piece(Sides::White, Pieces::Queen)
                    - board.count_piece(Sides::Black, Pieces::Queen))
            + 500
                * (board.count_piece(Sides::White, Pieces::Rook)
                    - board.count_piece(Sides::Black, Pieces::Rook))
            + 300
                * (board.count_piece(Sides::White, Pieces::Knight)
                    - board.count_piece(Sides::Black, Pieces::Knight))
            + 300
                * (board.count_piece(Sides::White, Pieces::Bishop)
                    - board.count_piece(Sides::Black, Pieces::Bishop))
            + 100
                * (board.count_piece(Sides::White, Pieces::Pawn)
                    - board.count_piece(Sides::Black, Pieces::Pawn)));

    material + (pawn_structure)
}

pub fn bot_move(board: &Board, depth: usize, side: Sides) -> (i32, Option<Move>) {
    let mut board = board.clone();
    let x = match side {
        Sides::White => maxi(&mut board, depth, side, i32::MIN, i32::MAX),
        Sides::Black => mini(&mut board, depth, side, i32::MIN, i32::MAX),
    };

    unsafe {
        println!("Count : {}", COUNT);
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
        //println!("Reached Depth 0 , score : {}" , evaluate(board, side.other()));
        return (evaluate(board, side), None);
    };

    let mut max = i32::MIN;
    let mut best_move = None;
    for mv in board.legal_moves(side) {
        unsafe {
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

        alpha = alpha.max(score);
        if beta <= alpha {
            break;
        }
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
        //println!("Reached Depth 0 , score : {}" , evaluate(board, side.other()));
        return (evaluate(board, side), None);
    };

    let mut min = i32::MAX;
    let mut best_move = None;
    for mv in board.legal_moves(side) {
        unsafe {
            COUNT += 1;
        }
        if !mv.apply(board) {
            continue;
        }
        //println!("Score : {} , move : {}" , evaluate(board, side) , mv);
        let (score, _) = maxi(board, depth - 1, side.other(), alpha, beta);
        mv.undo(board);

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

/*fn merge_sort(mvs : &Vec<Move>) -> Vec<Move> {

    todo!()
}*/
