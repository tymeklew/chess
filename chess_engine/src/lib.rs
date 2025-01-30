#![allow(dead_code)]

mod board;
mod bootstrap;
mod attacks;
mod game;
mod state;
mod square;

#[cfg(test)]
mod tests {
    use crate::{board::BitBoard, bootstrap::{BISHOP_ATTACKS, QUEEN_ATTACKS, ROOK_ATTACKS}, game::{Game, Square, FULL_COL}};

    #[test]
    fn it_works() {
        println!("{}" , BitBoard(ROOK_ATTACKS[35]));
        println!("{}" , BitBoard(QUEEN_ATTACKS[35]));
        /*let mut game = Game::new();
        game.init();

        game.legal_moves(Square::new(4, 0));*/
    }
}

// R N B Q K B N R
