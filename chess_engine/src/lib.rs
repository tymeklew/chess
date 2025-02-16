mod attacks;
mod board;
mod game;
mod pieces;
mod square;

#[cfg(test)]
mod tests {
    use crate::game::{Game, Move};
    use crate::square::Square;

    #[test]
    fn it_works() {
        let mut game = Game::new();
        game.init_board();

        game.display();
        let real = game.make_move(Move::new(Square::new(0, 1), Square::new(0, 3)));
        println!("{}", real);
        game.display();

        let mvd = game.pseudo_legal_moves(crate::pieces::Sides::White);
        for mv in mvd {
            println!("{}", mv.1);
        }
    }
}
