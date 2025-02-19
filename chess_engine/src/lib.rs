mod attacks;
mod board;
mod bot;
mod game;
mod moves;
mod pieces;
mod square;
mod tree;

pub use bot::mv;
pub use moves::Move;
pub use moves::{BasicMove , Capture};
pub use square::Square;

#[cfg(test)]
mod tests {
    use std::{i32, io::Read, time::Instant};

    use crate::{
        board::{self, Bitboard, Board}, bot::{self, evaluate, maxi, mini}, game::Game, moves::{BasicMove, Move}, pieces::Sides, square::Square
    };

    #[test]
    fn it_works() {
        
    }
}
