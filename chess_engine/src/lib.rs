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
pub use game::GameStatus;
pub use square::Square;

#[cfg(test)]
mod tests {
    use crate::{board::Board, bot::evaluate, bot_move, ChessGame, Sides};

    #[test]
    fn promotion() {
        let fen = String::from("1b6/P6k/8/8/8/8/8/K7 w - - 0 1");
        let mut board = Board::from_fen(fen);

        /*for mv in board.legal_moves(Sides::White) {
            println!("Promotions : {}", mv);
            println!("Before : ");
            board.display();
            mv.apply(&mut board);
            println!("After");
            board.display();
            mv.undo(&mut board);
            println!("Undo");
            board.display();
        }*/
    }

    #[test]
    fn checkmate() {
        let fen = String::from("8/8/8/8/8/5K2/6Q1/7k b - - 0 1");
        let board = Board::from_fen(fen);
        //board.display();

        assert_eq!(board.is_checkmate(Sides::Black), true);
    }

    #[test]
    fn stalemate() {
        let fen = String::from("8/8/8/8/8/8/5R2/5K1k b - - 0 1");
        let board = Board::from_fen(fen);
        //board.display();

        assert_eq!(board.is_stalemate(Sides::Black), true);
    }

    #[test]
    fn castle() {}

    #[test]
    fn capture() {
        let fen = String::from("8/8/4k3/8/6N1/2R1n3/3P1B2/K1Q2b2 w - - 0 1");
        let board = Board::from_fen(fen);
        //board.display();

        for mv in board.legal_moves(Sides::White) {
            //println!("{}", mv);
        }
    }

    #[test]
    fn uci() {
        let game = ChessGame::new();
        let str = "e2e3";
        let mv = game.move_from_uci(str);

        //println!("{:?}", mv);
    }

    #[test]
    fn real() {
        let game = ChessGame::new();

let mv =         bot_move(&game.board(), 2, Sides::Black);
println!("{}" , mv.1.unwrap());
    }
}
