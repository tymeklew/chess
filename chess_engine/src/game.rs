use crate::board::Board;
use crate::{Move, Sides};

#[derive(Clone, Copy , PartialEq)]
pub enum GameStatus {
    InProgress,
    // Side is winning side
    Checkmate(Sides),
    Statemate
}

pub struct ChessGame {
    turn: Sides,
    board: Board,
    status : GameStatus
}

impl ChessGame {
    pub fn new() -> Self {
        ChessGame {
            board: Board::new(),
            turn: Sides::White,
            status : GameStatus::InProgress,
        }
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn mv(&mut self, m: Move)
    {
        if self.status != GameStatus::InProgress {
            return;
        }
        m.apply(&mut self.board);
        self.turn = self.turn.other();

        if self.board.is_checkmate(self.turn) {
            self.status = GameStatus::Checkmate(self.turn);
        } else if self.board.is_stalemate(self.turn) {
            self.status = GameStatus::Statemate;
        }
    }
    pub fn move_from_uci(&self, input: &str) -> Option<Move>
    {
        self.board.move_from_uci(input)
    }
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn legal_moves(&self , side : Sides) -> Vec<Move> {
        self.board.legal_moves(side)
    }
}
