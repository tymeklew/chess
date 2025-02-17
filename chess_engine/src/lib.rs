mod attacks;
mod board;
mod game;
mod pieces;
mod moves;
mod square;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::game::Game;


    #[test]
    fn it_works() {
        let game = Game::new();
        let start = Instant::now();
        let x =         game.pseudo_legal_moves(crate::pieces::Sides::White);
        let diff = start.elapsed();
        println!("Time taken: {:?}", diff);
        for i in x {
            println!("{}", i);
        }
    }
}
