use crate::attacks::{pawn_moves, sliding_attacks, step_attacks};
use crate::moves::{BasicMove, Capture, Castle, Move, Promotion};
use crate::pieces::{self, Pieces, Sides, ALL_PIECES, ALL_SIDES, PIECES_COUNT, SIDES_COUNT};
use crate::square::Square;
use core::panic;
use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Clone)]
pub struct MoveRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

impl Default for MoveRights {
    fn default() -> Self {
        MoveRights {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }
}

impl MoveRights {
    pub fn new() -> MoveRights {
        MoveRights {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; PIECES_COUNT]; SIDES_COUNT],
    pub sides: [Bitboard; SIDES_COUNT],
    pub move_rights : MoveRights,
}

impl Board {
    pub fn occupied(&self) -> Bitboard {
        self.sides[Sides::White] | self.sides[Sides::Black]
    }
    pub fn display(&self) {
        let occupied = self.occupied();
        for i in (0..8).rev() {
            for j in 0..8 {
                let idx = i * 8 + j;
                if Bitboard(1 << idx) & occupied != Bitboard(0) {
                    let piece = self.get_piece(Square::from_idx(idx));
                    print!(
                        "{}",
                        match self.get_side(Square::from_idx(idx)) {
                            Sides::White => match piece {
                                Pieces::Pawn => "♟",
                                Pieces::Rook => "♜",
                                Pieces::Knight => "♞",
                                Pieces::Bishop => "♝",
                                Pieces::Queen => "♛",
                                Pieces::King => "♚",
                            },
                            Sides::Black => match piece {
                                Pieces::Pawn => "♙",
                                Pieces::Rook => "♖",
                                Pieces::Knight => "♘",
                                Pieces::Bishop => "♗",
                                Pieces::Queen => "♕",
                                Pieces::King => "♔",
                            },
                        }
                    );
                    continue;
                }
                print!("+");
            }
            println!();
        }
    }

    pub fn empty(&self) -> Bitboard {
        !self.occupied()
    }

    pub fn enemy(&self, side: Sides) -> Bitboard {
        self.sides[side.other()]
    }

    pub fn friendly(&self, side: Sides) -> Bitboard {
        self.sides[side]
    }

    pub fn get_piece(&self, pos: Square) -> Pieces {
        let side = self.get_side(pos);

        for piece in ALL_PIECES {
            if self.pieces[side][piece].0 & (1 << pos.idx()) != 0 {
                return piece;
            }
        }

        return Pieces::Pawn;
    }

    pub fn get_side(&self, pos: Square) -> Sides {
        let idx = pos.idx();

        for side in ALL_SIDES {
            if self.sides[side].0 & (1 << idx) != 0 {
                return side;
            }
        }

        // Need to fix
        Sides::White
    }

    pub fn is_check(&self, side: Sides) -> bool {
        let mvs = self.pseudo_legal_moves(side.other());

        mvs.iter().any(|f| match f.capture() {
            Some(Pieces::King) => true,
            _ => false,
        })
    }

    pub fn is_checkmate(&self, side: Sides) -> bool {
        self.is_check(side) && self.legal_moves(side).is_empty()
    }

    pub fn is_stalemate(&self, side: Sides) -> bool {
        !self.is_check(side) && self.legal_moves(side).is_empty()
    }

    pub fn new() -> Self {
        let mut board = Board::default();

        board.pieces[Sides::White][Pieces::Pawn] |= Bitboard(255 << 8);
        board.pieces[Sides::Black][Pieces::Pawn] |= Bitboard(255 << (8 * 6));

        board.pieces[Sides::White][Pieces::Rook] |= Bitboard(1 | 1 << 7);
        board.pieces[Sides::Black][Pieces::Rook] |= Bitboard(1 << (8 * 7) | 1 << (8 * 7 + 7));

        board.pieces[Sides::White][Pieces::Knight] |= Bitboard(1 << 1 | 1 << 6);
        board.pieces[Sides::Black][Pieces::Knight] |= Bitboard(1 << (8 * 7 + 1) | 1 << (8 * 7 + 6));

        board.pieces[Sides::White][Pieces::Bishop] |= Bitboard(1 << 2 | 1 << 5);
        board.pieces[Sides::Black][Pieces::Bishop] |= Bitboard(1 << (8 * 7 + 2) | 1 << (8 * 7 + 5));

        board.pieces[Sides::White][Pieces::Queen] |= Bitboard(1 << 3);
        board.pieces[Sides::Black][Pieces::Queen] |= Bitboard(1 << (8 * 7 + 4));

        board.pieces[Sides::White][Pieces::King] |= Bitboard(1 << 4);
        board.pieces[Sides::Black][Pieces::King] |= Bitboard(1 << (8 * 7 + 3));

        board.sides[Sides::White] = board.pieces[Sides::White]
            .iter()
            .fold(Bitboard(0), |acc, x| acc | *x);
        board.sides[Sides::Black] = board.pieces[Sides::Black]
            .iter()
            .fold(Bitboard(0), |acc, x| acc | *x);

        board
    }

    pub fn legal_moves(&self, side_to_move: Sides) -> Vec<Box<dyn Move>> {
        self.pseudo_legal_moves(side_to_move)
            .into_iter()
            .filter(|f| {
                let mut new = self.clone();
                f.apply(&mut new);
                !new.is_check(side_to_move)
            })
            .collect()
    }

    // Generates moves including pawn moves
    fn pseudo_legal_moves(&self, side_to_move: Sides) -> Vec<Box<dyn Move>> {
        const ROOK_RAY_INDEX: [usize; 4] = [0, 1, 4, 5];
        const BISHOP_RAY_INDEX: [usize; 4] = [2, 3, 6, 7];
        const KNIGHT_DELTAS: [i8; 8] = [15, 17, 10, 6, -15, -17, -10, -6];
        const KING_DELTAS: [i8; 8] = [1, -1, 8, -8, 7, -7, 9, -9];
        const WHITE_PAWN_DELTAS: [i8; 2] = [7, 9];
        const BLACK_PAWN_DELTAS: [i8; 2] = [-7, -9];
        const WHITE_PROMOTION_ROW: Bitboard = Bitboard(0xFF << (8 * 7));
        const BLACK_PROMOTION_ROW: Bitboard = Bitboard(0xFF);
        const QUEEN_SIDE_CASTLE: Bitboard = Bitboard(0b1110);
        const KING_SIDE_CASTLE: Bitboard = Bitboard(0b01100000);

        let occupied = self.occupied();
        let mut moves: Vec<Box<dyn Move>> = Vec::new();

        for piece in ALL_PIECES {
            for i in 0..64 {
                let piece_bb = self.pieces[side_to_move][piece];
                if piece_bb.0 & (1 << i) == 0 {
                    continue;
                }

                let bb = match piece {
                    Pieces::Pawn => {
                        let bb = match side_to_move {
                            Sides::Black => step_attacks(i, &BLACK_PAWN_DELTAS),
                            Sides::White => step_attacks(i, &WHITE_PAWN_DELTAS),
                        };
                        bb & self.enemy(side_to_move)
                            | (pawn_moves(i, side_to_move, occupied) & !self.occupied())
                    }
                    Pieces::Rook => sliding_attacks(i, occupied, &ROOK_RAY_INDEX),
                    Pieces::Knight => step_attacks(i, &KNIGHT_DELTAS),
                    Pieces::Bishop => sliding_attacks(i, occupied, &BISHOP_RAY_INDEX),
                    Pieces::Queen => {
                        sliding_attacks(i, occupied, &ROOK_RAY_INDEX)
                            | sliding_attacks(i, occupied, &BISHOP_RAY_INDEX)
                    }
                    Pieces::King => step_attacks(i, &KING_DELTAS),
                };

                if bb.0 == 0 {
                    continue;
                }

                let basic_moves = bb & !self.occupied();
                let captures = bb & self.enemy(side_to_move);

                for j in 0..64 {
                    if basic_moves.0 & (1 << j) != 0 {
                        if piece != Pieces::Pawn {
                            moves.push(Box::new(BasicMove::new(
                                Square::from_idx(i),
                                Square::from_idx(j),
                            )));
                            continue;
                        }

                        let promotion_row = match side_to_move {
                            Sides::White => WHITE_PROMOTION_ROW,
                            Sides::Black => BLACK_PROMOTION_ROW,
                        };

                        if promotion_row.0 & (1 << j) != 0 {
                            for promotion_piece in &ALL_PIECES[1..5] {
                                moves.push(Box::new(Promotion::new(
                                    Square::from_idx(i),
                                    Square::from_idx(j),
                                    *promotion_piece,
                                )));
                            }
                        }
                    }
                }
                // Add promotions to pawns

                for j in 0..64 {
                    if captures.0 & (1 << j) != 0 {
                        let captured_piece = self.get_piece(Square::from_idx(j));
                        moves.push(Box::new(Capture::new(
                            Square::from_idx(i),
                            Square::from_idx(j),
                            captured_piece,
                        )));
                    }
                }

                // Add castling logic
                if piece == Pieces::King {
                    if side_to_move == Sides::White {
                        if (occupied.0 & KING_SIDE_CASTLE.0 == 0) & self.move_rights.white_king_side {
                            moves.push(Box::new(Castle::new(Sides::White, true)));
                        } else if (occupied.0 & QUEEN_SIDE_CASTLE.0 == 0) & self.move_rights.white_queen_side {
                            moves.push(Box::new(Castle::new(Sides::White, false)));
                        }
                    } else {
                        if (occupied.0 & (KING_SIDE_CASTLE.0 << (8 * 7)) == 0) & self.move_rights.black_king_side {
                            moves.push(Box::new(Castle::new(Sides::Black, true)));
                        } else if (occupied.0 & (QUEEN_SIDE_CASTLE.0 << (8 * 7)) == 0) & self.move_rights.black_queen_side{
                            moves.push(Box::new(Castle::new(Sides::Black, false)));
                        }
                    }
                }
            }
        }
        moves
    }

    pub fn count_piece(&self, side: Sides, piece: Pieces) -> i32 {
        self.pieces[side][piece].0.count_ones() as i32
    }

    pub fn place_piece(&mut self, side: Sides, piece: Pieces, square: Square) {
        self.sides[side] ^= Bitboard(1 << square.idx());
        self.pieces[side][piece] ^= Bitboard(1 << square.idx());
    }

    pub fn from_fen(input: String) -> Board {
        let mut board = Board::default();
        let parts: Vec<&str> = input.split(" ").collect();

        // Deal with placement
        for (rank, row) in parts.get(0).unwrap().split("/").enumerate() {
            let mut file: usize = 0;
            for c in row.chars() {
                if c.is_digit(10) {
                    file += c.to_digit(10).unwrap() as usize;
                    continue;
                }

                let side = if c.is_uppercase() {
                    Sides::White
                } else {
                    Sides::Black
                };

                match c.to_ascii_lowercase() {
                    'p' => board
                        .place_piece(side, Pieces::Pawn, Square::new(file,  7 - rank)),
                    'r' => board
                        .place_piece(side, Pieces::Rook, Square::new(file, 7 - rank)),
                    'n' => board
                        .place_piece(side, Pieces::Knight, Square::new(file, 7 - rank)),
                    'b' => board
                        .place_piece(side, Pieces::Bishop, Square::new(file, 7 - rank)),
                    'q' => board
                        .place_piece(side, Pieces::Queen, Square::new(file, 7 - rank)),
                    'k' =>board
                        .place_piece(side, Pieces::King, Square::new(file, 7 - rank)),
                    _ => panic!("Invalid piece {}" , c.to_ascii_lowercase()),
                }
                file += 1;
            }
        }

        // TODO
        // Turn
        /*match parts.get(1) {
            Some(&"w") => game.turn = Sides::White,
            Some(&"b") => game.turn = Sides::Black,
            _ => panic!("Invalid turn"),
        }*/

        // Castling
        for c in parts.get(2).unwrap().chars() {
            match c {
                'K' => board.move_rights.white_king_side = true,
                'Q' => board.move_rights.white_queen_side = true,
                'k' => board.move_rights.black_king_side = true,
                'q' => board.move_rights.black_queen_side = true,
                '-' => break,
                _ => panic!("Invalid castling rights"),
            }
        }

        // En Pessant (not implemented)

        // Halfmove clock (not implemented)

        board
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: [[Bitboard(0); PIECES_COUNT]; SIDES_COUNT],
            sides: [Bitboard(0); SIDES_COUNT],
            move_rights: MoveRights::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bitboard(pub u64);

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard(0)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..8).rev() {
            // Reverse row order
            for j in 0..8 {
                write!(f, "{}", (self.0 >> (i * 8 + j)) & 1)?; // No longer 63 - (i * 8 + j)
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
