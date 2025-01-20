#![allow(dead_code)]

mod board;
mod game;
mod state;

mod tests {
    use crate::{
        board::BitBoard,
        game::{Game, Pieces, Sides, Square},
    };
    use std::fmt::Display;

    #[test]
    fn it_works() {
        let mut game = Game::new();
        game.init();

        let x = game.legal_moves(Square::new(0, 0));
    }
}
