mod board;
mod game;
mod pieces;

pub use board::Board;
pub use game::Game;
pub use game::Move;
pub use game::Position;
pub use pieces::Colour;
pub use pieces::PieceType;

#[cfg(test)]
mod tests {

    use game::{Game, Move, Position};

    use super::*;

    #[test]
    fn it_works() {
        // let game = Game::new();
        // Generate a default board with the white and black pieces made
        // Get input -> mov
        // game.move(Move::Basic((0 , 2) , ())
        // Inside Game
        // - get the piece type
        // - PieceType::(x).is_legal_move(mov);
        // - Only then will it move
        // - Returns true if move was legal and scucesfull and false if it was not legal
        // pos!(0 , 0)

        let mut game = Game::new();
        println!("{}", game.board);

        println!("{:?}", game.available_moves(Position::new(0, 1)));

        game.mv(Move::b(0, 1, 0, 3));

        game.mv(game::Move::Basic(Position::new(1, 6), Position::new(1, 4)));

        assert_eq!(1, 1);
    }
}
