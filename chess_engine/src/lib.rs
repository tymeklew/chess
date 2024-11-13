mod board;
mod game;

mod tests {
    use crate::{
        board::BitBoard,
        game::{Game, Pieces, Sides, Square},
    };

    #[test]
    fn it_works() {
        let x: u64 = 2;
        let mut game = Game::new();
        game.init();

        game.legal_moves(Square::new(1, 1));
    }
}
