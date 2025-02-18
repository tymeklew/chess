mod attacks;
mod board;
mod bot;
mod game;
mod pieces;
mod moves;
mod square;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{board::Board, bot, game::Game};


    #[test]
    fn it_works() {
        let board = Board::new();

let x =         bot::minimax(&board, 1, true);
println!("{}" , x);
    }
}
