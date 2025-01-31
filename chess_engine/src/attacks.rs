use crate::board::Bitboard;

pub static RAY_ATTACKS : [[u64 ; 64] ; 8] = init_rays();
const RAYS : [i8 ; 8] = [1 , -1 , 7 , -7 , 8 , -8 , 9 , -9];

const fn init_rays() -> [[u64 ; 64] ; 8] {
    let mut rays = [[0; 64]; 8];

    let mut i = 0;
    while i < 8 {
        let mut j : usize  = 0;

        while j < 64 {
            let mut sqr = j as i8;
            loop {
                let next = sqr + RAYS[i];

                let file_diff = (sqr & 0x7) - (next & 0x7);
                if next >= 64 || next <= 0 || file_diff.abs() > 1 {
                    break;
                }

                rays[i][j] |= 1 << next;
                sqr = next;
            }
            j += 1;
        }
        i += 1;
    }
    rays
}


/// Classic Generalized Ray Sliding Piece Attack Generator
/// https://www.chessprogramming.org/Classical_Approach#Conditional_3
fn sliding_attacks(square : usize , occupied : Bitboard, ray_indecies : &[usize]) -> Bitboard {
    let mut attacks = Bitboard(0);
    for i in ray_indecies {
        let blockers = occupied.0 & RAY_ATTACKS[*i][square];

        if blockers == 0 {
            attacks |= Bitboard(RAY_ATTACKS[*i][square]);
            continue;
        } 

        let blocker = match RAYS[*i].is_positive() {
            true => blockers.trailing_zeros() ,
            false => 63 - blockers.leading_zeros(),
        } as usize;

        attacks |= Bitboard(RAY_ATTACKS[*i][square] ^ RAY_ATTACKS[*i][blocker]);
    }
    attacks
}

fn step_attacks(square : usize , deltas : &[i8]) -> Bitboard {
    let mut attacks = Bitboard(0);

    for delta in deltas {
        let next = square as i8 + *delta as i8;
        if next >= 0 && next < 64 {
            attacks |= Bitboard(1 << next);
        }

    }
    attacks
} 


const ROOK_RAY_INDEX : [usize ; 4] = [0 , 1 ,4 , 5];
pub fn rook_attacks(square : usize , occupied : Bitboard) -> Bitboard {
    sliding_attacks(square , occupied , &ROOK_RAY_INDEX)
}

const BISHOP_RAY_INDEX : [usize ; 4] = [2 , 3 , 6 , 7];
pub fn bishop_attacks(square : usize , occupied : Bitboard) -> Bitboard {
    sliding_attacks(square , occupied , &BISHOP_RAY_INDEX)
}

pub fn queen_attacks(square : usize , occupied : Bitboard) -> Bitboard {
    rook_attacks(square , occupied) | bishop_attacks(square , occupied)
}

const KNIGHT_DELTAS : [i8 ; 8] = [15 , 17 , 10 , 6 , -15 , -17 , -10 , -6];
pub fn knight_attacks(square : usize) -> Bitboard {
    step_attacks(square , &KNIGHT_DELTAS)
}

const KING_DELTAS : [i8 ; 8] = [1 , -1 , 8 , -8 , 7 , -7 , 9 , -9];
pub fn king_attacks(square : usize) -> Bitboard {
    step_attacks(square , &KING_DELTAS)
}

const WHITE_PAWN_DELTAS : [i8 ; 2] = [7 , 9];
pub fn white_pawn_attacks(square : usize) -> Bitboard {
    step_attacks(square , &WHITE_PAWN_DELTAS)
}

const BLACK_PAWN_DELTAS : [i8 ; 2] = [-7 , -9];
pub fn black_pawn_attacks(square : usize) -> Bitboard {
    step_attacks(square , &BLACK_PAWN_DELTAS)
}