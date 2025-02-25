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
    use crate::{board::Board, bot::evaluate, bot_move, ChessGame};

    #[test]
    fn promotion() {}

    #[test]
    fn checkmate() {}

    #[test]
    fn stalemate() {}

    #[test]
    fn castle() {}

    #[test]
    fn capture() {
    }

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
        game.board().display();
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mv = game.move_from_uci(&input.trim()).unwrap();
            game.mv(mv);
            game.board().display();

            println!("\n\n\n\n\n\n\n\n");
            let bot = bot_move(&mut game.board(), 2, crate::Sides::Black);
            let mv = bot.1.unwrap();
            println!("Score : {} : {}" , bot.0 , mv);
            game.mv(mv);
            game.board().display();


        }
    }
}
