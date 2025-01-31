use crate::board::Bitboard;
use crate::pieces::{Pieces, Sides, PIECES_COUNT, SIDES_COUNT};

pub struct Game {
    pieces: [[Bitboard; PIECES_COUNT]; SIDES_COUNT],
    sides : [Bitboard; SIDES_COUNT],
}

impl Game {
    pub fn new() -> Self {
        Game {
            pieces: Default::default(),
            sides: Default::default(),
        }
    }

    pub fn init_board(&mut self) { 
        self.pieces[Sides::White][Pieces::Pawn] |= Bitboard(255 << 8); 
        self.pieces[Sides::Black][Pieces::Pawn] |= Bitboard(255 << (8 * 6));

        self.pieces[Sides::White][Pieces::Rook] |= Bitboard(1 | 1 << 7);
        self.pieces[Sides::Black][Pieces::Rook] |= Bitboard(1 << (8 * 7) | 1 << (8 * 7 + 7));        

        self.pieces[Sides::White][Pieces::Knight] |= Bitboard(1 << 1 | 1 << 6);
        self.pieces[Sides::Black][Pieces::Knight] |= Bitboard(1 << (8 * 7 + 1) | 1 << (8 * 7 + 6));

        self.pieces[Sides::White][Pieces::Bishop] |= Bitboard(1 << 2 | 1 << 5);
        self.pieces[Sides::Black][Pieces::Bishop] |= Bitboard(1 << (8 * 7 + 2) | 1 << (8 * 7 + 5));

        self.pieces[Sides::White][Pieces::Queen] |= Bitboard(1 << 3);
        self.pieces[Sides::Black][Pieces::Queen] |= Bitboard(1 << (8 * 7 + 4));

        self.pieces[Sides::White][Pieces::King] |= Bitboard(1 << 4);
        self.pieces[Sides::Black][Pieces::King] |= Bitboard(1 << (8 * 7 + 3));

        self.sides[Sides::White] = self.pieces[Sides::White].iter().fold(Bitboard(0), |acc , x| acc | *x);
        self.sides[Sides::Black] = self.pieces[Sides::Black].iter().fold(Bitboard(0), |acc , x| acc | *x);
    }

    pub fn occupied(&self) -> Bitboard {
        self.sides[Sides::White] | self.sides[Sides::Black]
    }

    pub fn pseudo_legal_moves(&self) -> Bitboard {
        todo!()
    }
}