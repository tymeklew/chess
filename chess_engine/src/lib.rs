mod attacks;
mod board;
mod game;
mod pieces;
mod square;

#[cfg(test)]
mod tests {
    use crate::{attacks::pawn_moves, board::Bitboard, game::{Game, Move}};
    use crate::square::Square;

    #[test]
    fn it_works() {
        let mut game = Game::new();
        game.init_board();

        game.display();
        let real = game.make_move(Move::new(Square::new(0 , 1) , Square::new(0 , 3)));
        println!(
            "{}" , real
        );
        game.display();

        let pos = Square::new( 0 , 3);
        println!("{}" , pos.to_algebraic());
    }
}
