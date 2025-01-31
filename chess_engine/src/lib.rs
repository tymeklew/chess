mod game;
mod pieces;
mod attacks;
mod board;

#[cfg(test)]
mod tests {
    use crate::{attacks::{bishop_attacks, black_pawn_attacks, king_attacks, knight_attacks, queen_attacks, rook_attacks, white_pawn_attacks, RAY_ATTACKS}, board::Bitboard, game::Game};


    #[test]
    fn it_works() {
        let mut game = Game::new();
        game.init_board();

    }
}

// R N B Q K B N R
