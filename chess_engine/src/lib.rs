mod attacks;
mod board;
mod game;
mod pieces;

#[cfg(test)]
mod tests {
    use crate::{attacks::pawn_moves, board::Bitboard, game::{Game, Move}};

    #[test]
    fn it_works() {
        let mut game = Game::new();
        game.init_board();

        game.display();
        let real = game.make_move(Move::new(8 , 8 * 3));
        println!(
            "{}" , real
        );
        game.display();

    }
}
