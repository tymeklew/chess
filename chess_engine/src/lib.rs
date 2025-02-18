mod attacks;
mod board;
mod bot;
mod game;
mod moves;
mod pieces;
mod square;
mod tree;

#[cfg(test)]
mod tests {
    use std::{i32, time::Instant};

    use crate::{
        board::{self, Board},
        bot::{self, generate_tree},
        game::Game,
        moves::{BasicMove, Move},
        pieces::Sides,
        square::Square,
        tree::Node,
    };

    #[test]
    fn it_works() {
        let board = Board::new();


                let mut tree = generate_tree(&board, 0, 5, Sides::White);

        let mut root = Node::new(i32::MAX, None);
        root.add(tree);

        println!("Finished started counting");
        println!("{}", root.count());
    }
}
