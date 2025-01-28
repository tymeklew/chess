#![allow(dead_code)]

mod board;
mod bootstrap;
mod game;
mod state;

#[cfg(test)]
mod tests {
    use crate::game::{Square , Game};

    #[test]
    fn it_works() {
        let mut game = Game::new();
        game.init();

        game.legal_moves(Square::new(4, 0));
    }
}

// R N B Q K B N R
