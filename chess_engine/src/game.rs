use crate::board::Board;
use crate::{Move, Sides, Square};

#[derive(Clone, Copy, PartialEq)]
pub enum GameStatus {
    InProgress,
    // Side is winning side
    Checkmate(Sides),
    Statemate,
}

pub struct ChessGame {
    turn: Sides,
    board: Board,
    status: GameStatus,
    history: Vec<Move>,
}

lazy_static::lazy_static!(
    pub static ref WHITE_QUEEN_SIDE_ROOK: Square = Square::new(0, 0);
    pub static ref WHITE_KING: Square = Square::new(4 , 0);
    pub static ref WHITE_KING_SIDE_ROOK: Square = Square::new(7 , 0);

    pub static ref BLACK_QUEEN_SIDE_ROOK : Square = Square::new(0 , 7);
    pub static ref BLACK_KING : Square = Square::new(4 , 7);
    pub static ref BLACK_KING_SIDE_ROOK : Square = Square::new(7 , 7);
);

impl ChessGame {
    pub fn new() -> Self {
        ChessGame {
            board: Board::new(),
            turn: Sides::White,
            status: GameStatus::InProgress,
            history: Vec::new(),
        }
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn is_over(&self) -> bool {
        self.status != GameStatus::InProgress
    }

    // Funcion to retun all the moves played in UCI seperated by comma
    pub fn all_moves(&self) -> String {
        let mut moves = String::new();

        for mv in self.history.iter() {
            moves.push_str(&mv.into_uci(Sides::White));
            moves.push(',');
        }

        moves
    }

    pub fn mv(&mut self, m: Move) {
        if self.status != GameStatus::InProgress {
            return;
        }
        // Update move rights if king moves side is disabled to castling if rook moves side is disabled
        if m.source() == *WHITE_KING {
            self.board.move_rights.white_king_side = false;
            self.board.move_rights.white_queen_side = false;
        } else if m.source() == *WHITE_QUEEN_SIDE_ROOK {
            self.board.move_rights.white_queen_side = false;
        } else if m.source() == *WHITE_KING_SIDE_ROOK {
            self.board.move_rights.white_king_side = false;
        } else if m.source() == *BLACK_KING {
            self.board.move_rights.black_king_side = false;
            self.board.move_rights.black_queen_side = false;
        } else if m.source() == *BLACK_QUEEN_SIDE_ROOK {
            self.board.move_rights.black_queen_side = false;
        } else if m.source() == *BLACK_KING_SIDE_ROOK {
            self.board.move_rights.black_king_side = false;
        }

        m.apply(&mut self.board);
        self.history.push(m);
        self.turn = self.turn.other();
        // Check if the game is over
        self.update_status();
    }

    pub fn history(&self) -> Vec<Move> {
        self.history.clone()
    }

    pub fn update_status(&mut self) {
        if self.board.is_checkmate(self.turn) {
            println!("Checkmate");
            self.status = GameStatus::Checkmate(self.turn);
        } else if self.board.is_stalemate(self.turn) {
            self.status = GameStatus::Statemate;
        }
    }
    pub fn move_from_uci(&self, input: &str) -> Option<Move> {
        self.board.move_from_uci(input)
    }
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn legal_moves(&self, side: Sides) -> Vec<Move> {
        self.board.legal_moves(side)
    }
}
