mod attacks;
mod board;
mod game;
mod pieces;

#[cfg(test)]
mod tests {
    use crate::game::Game;

    #[test]
    fn it_works() {
        let mut game = Game::new();
        game.init_board();

        let legals = game.pseudo_legal_moves(crate::pieces::Sides::White);

        for l in legals {
            println!("{}", l);
        }
    }
}

// R N B Q K B N R
