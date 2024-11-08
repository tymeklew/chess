use crate::{
    game::Position,
    pieces::{Colour, Piece, PieceType},
};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Board([[Option<Piece>; 8]; 8]);

impl Board {
    // These 2 methods are to convert the array to how it is displayed on the board
    pub fn set(&mut self, pos: &Position, new: Option<Piece>) {
        self.0[pos.row as usize][pos.col as usize] = new;
    }
    pub fn get(&self, pos: &Position) -> Option<Piece> {
        self.0[pos.row as usize][pos.col as usize]
    }

    pub fn full(&self) -> [[Option<Piece>; 8]; 8] {
        self.0.clone()
    }
    // A piece has moved
    pub fn moved(&mut self, pos: &Position) {
        if let Some(piece) = self.0[pos.row as usize][pos.col as usize].as_mut() {
            piece.moved()
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board([[None; 8]; 8]);

        for i in 0..8 {
            board.set(
                &Position::new(i, 1),
                Some(Piece::new(Colour::White, PieceType::Pawn)),
            );
            board.set(
                &Position::new(i, 6),
                Some(Piece::new(Colour::Black, PieceType::Pawn)),
            );
        }

        let ranks = [0, 7];
        let colour = [Colour::White, Colour::Black];

        for (i, piece) in [PieceType::Rook, PieceType::Knight, PieceType::Bishop]
            .iter()
            .enumerate()
        {
            for j in 0..2 {
                board.set(
                    &Position::new(i as i8, ranks[j]),
                    Some(Piece::new(colour[j], *piece)),
                );
                board.set(
                    &Position::new(7 - i as i8, ranks[j]),
                    Some(Piece::new(colour[j], *piece)),
                );
            }
        }

        for i in 0..2 {
            board.set(
                &Position::new(3, ranks[i]),
                Some(Piece::new(colour[i], PieceType::Queen)),
            );
            board.set(
                &Position::new(4, ranks[i]),
                Some(Piece::new(colour[i], PieceType::King)),
            );
        }

        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arr = self.0;
        arr.reverse();
        for el in arr {
            for tile in el {
                write!(
                    f,
                    "{}",
                    match tile {
                        None => "X",
                        Some(piece) => match piece.piece_type() {
                            PieceType::King => "K",
                            PieceType::Queen => "Q",
                            PieceType::Rook => "R",
                            PieceType::Bishop => "B",
                            PieceType::Knight => "N",
                            PieceType::Pawn => "P",
                        },
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        write!(f, "Hello")
    }
}
