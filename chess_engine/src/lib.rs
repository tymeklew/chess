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
pub use moves::{BasicMove, Capture};
pub use pieces::Sides;
pub use square::Square;

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use crate::board::Bitboard;
    use crate::moves::Promotion;
    use crate::pieces::{Pieces, Sides};
    use crate::Move;
    use crate::Square;
    use crate::{board::Board, bot_move};
    use crate::{Capture, ChessGame};

    #[test]
    fn promotion() {
        let fen = "7k/P7/8/8/8/8/8/K7 w - - 0 1".to_string();
        let mut board = Board::from_fen(fen);

        let moves = board.legal_moves(Sides::White);

        // 4 moves for promotion & 3 moves for king
        assert_eq!(moves.len(), 7);

        let promotion_move = Promotion::new(Square::new(0, 6), Square::new(0, 7), Pieces::Queen);
        promotion_move.apply(&mut board);

        // Magic numer for queen and 2 kings in correct position
        assert_eq!(board.sides[Sides::White].0, 72057594037927937);
    }

    #[test]
    fn checkmate() {
        let fen = "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1".to_string();
        let board = Board::from_fen(fen);

        assert_eq!(board.is_checkmate(Sides::Black), true);

        let fen = "7k/5N2/8/8/3B4/3B4/8/K5R1 b - - 0 1".to_string();
        let board = Board::from_fen(fen);
        assert_eq!(board.is_checkmate(Sides::Black), true);

        let fen = "7k/8/8/8/8/3B4/8/K5R1 b - - 0 1".to_string();
        let board = Board::from_fen(fen);
        assert_eq!(board.is_checkmate(Sides::Black), false);
    }

    #[test]
    fn stalemate() {
        let fen = "7k/2R5/8/8/8/8/8/K5R1 w - - 0 1".to_string();
        let board = Board::from_fen(fen);

        assert_eq!(board.is_stalemate(Sides::Black), true);
    }

    #[test]
    fn castle() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1".to_string();
        let board = Board::from_fen(fen);

        let x = board.legal_moves(Sides::White);
        for i in x {
            //println!("{}", i);
        }
    }

    #[test]
    fn it_works() {
        let mut game = ChessGame::new();

        loop {
            std::io::stdin().read_line(&mut String::new()).unwrap();
            let white_mv = bot_move(game.board(), 5, Sides::White).1.unwrap();
            println!("White : {}", white_mv);
            game.boxed_mv(white_mv);

            game.board().display();

            std::io::stdin().read_line(&mut String::new()).unwrap();
            let black_mv = bot_move(game.board(), 5, Sides::Black).1.unwrap();
            println!("Black : {}", &black_mv);
            game.boxed_mv(black_mv);

            game.board().display();
        }
    }
}
