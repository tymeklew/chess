mod attacks;
mod board;
mod bot;
mod game;
mod moves;
mod pieces;
mod square;
mod tree;

pub use bot::bot_move;
pub use game::ChessGame;
pub use moves::Move;
pub use pieces::Sides;
pub use square::Square;

#[cfg(test)]
mod tests {
    use crate::ChessGame;

    #[test]
    fn promotion() {}

    #[test]
    fn checkmate() {}

    #[test]
    fn stalemate() {}

    #[test]
    fn castle() {}

    #[test]
    fn capture() {}

    #[test]
    fn uci() {
        let game = ChessGame::new();
        let str = "e2e3";
        let mv = game.move_from_uci(str);
        

        println!("{:?}", mv);
    }

    #[test]
    fn it_works() {
        let mut game = ChessGame::new();


        /*loop {
            std::io::stdin().read_line(&mut String::new()).unwrap();
            let white_mv = bot_move(game.board(), 3, Sides::White);
            let mv = white_mv.1.unwrap();
            println!("Score : {}", white_mv.0);
            println!("Move : {}", mv);
            game.boxed_mv(mv);
            game.board().display();

            std::io::stdin().read_line(&mut String::new()).unwrap();
            let black_mv = bot_move(game.board(), 3, Sides::Black).1.unwrap();
            game.boxed_mv(black_mv);
            game.board().display();
        }*/
    }
}
