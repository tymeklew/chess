mod board;
mod game;
mod pieces;

#[cfg(test)]
mod tests {

    use game::{Game, Position};

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

        game.mv(game::Move::Basic(
            Position { col: 0, row: 1 },
            Position { col: 0, row: 3 },
        ));

        let moves = game
            .board
            .get(Position::new(0, 3))
            .unwrap()
            .piece_type()
            .available_moves(Position::new(0, 3), &game.board);

        println!("{:?}", moves);

        println!("{}", game.board);

        assert_eq!(1, 1);
    }
}
