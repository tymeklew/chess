mod attacks;
mod board;
mod bot;
mod game;
mod moves;
mod pieces;
mod square;
mod tree;

pub use bot::bot_move;
pub use moves::Move;
pub use moves::{BasicMove , Capture};
pub use square::Square;
pub use game::ChessGame;
pub use pieces::Sides;

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use crate::ChessGame;
    use crate::{board::Board, bot_move};
    use crate::pieces::Sides;

    #[test]
    fn it_works() {
        let mut game = ChessGame::new();


       loop {
        std::io::stdin().read_line(&mut String::new()).unwrap();
        let white_mv = bot_move(game.board(), 4, Sides::White).1.unwrap();
        println!("White : {}" , white_mv);
        game.mv(&white_mv);

        game.board().display();

        std::io::stdin().read_line(&mut String::new()).unwrap();
        let black_mv = bot_move(game.board(), 4, Sides::Black).1.unwrap();
        println!("Black : {}" , &black_mv);
        game.mv(&black_mv);

        game.board().display();
       }
    }
}
