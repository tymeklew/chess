mod attacks;
mod board;
mod bot;
mod game;
mod moves;
mod pieces;
mod square;

pub use bot::bot_move;
pub use game::ChessGame;
pub use moves::Move;
pub use pieces::Sides;
pub use square::Square;

#[cfg(test)]
mod tests {
    use crate::{board::Board, bot::evaluate, bot_move, ChessGame, Sides};

    #[test]
    fn promotion() {}

    #[test]
    fn checkmate() {
        let fen = String::from("8/8/8/8/8/5K2/6Q1/7k b - - 0 1");
        let board = Board::from_fen(fen);
        board.display();

        assert_eq!(board.is_checkmate(Sides::Black), true);
    }

    #[test]
    fn stalemate() {
        let fen = String::from("8/8/8/8/8/8/5R2/5K1k b - - 0 1");
        let board = Board::from_fen(fen);
        board.display();

        assert_eq!(board.is_stalemate(Sides::Black), true);
    }

    #[test]
    fn castle() {}

    #[test]
    fn capture() {
        let fen = String::from("8/8/4k3/8/6N1/2R1n3/3P1B2/K1Q2b2 w - - 0 1");
        let board = Board::from_fen(fen);
        board.display();

        for mv in board.legal_moves(Sides::White) {
            //println!("{}", mv);
        }
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
        /*loop {
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


        }*/
    }
}
