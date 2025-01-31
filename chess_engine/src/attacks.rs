use crate::board::Bitboard;

pub static RAY_ATTACKS: [[u64; 64]; 8] = init_rays();
const RAYS: [i8; 8] = [1, -1, 7, -7, 8, -8, 9, -9];

const fn init_rays() -> [[u64; 64]; 8] {
    let mut rays = [[0; 64]; 8];

    let mut i = 0;
    while i < 8 {
        let mut j: usize = 0;

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
pub fn sliding_attacks(square: usize, occupied: Bitboard, ray_indecies: &[usize]) -> Bitboard {
    let mut attacks = Bitboard(0);
    for i in ray_indecies {
        let blockers = occupied.0 & RAY_ATTACKS[*i][square];

        if blockers == 0 {
            attacks |= Bitboard(RAY_ATTACKS[*i][square]);
            continue;
        }

        let blocker = match RAYS[*i].is_positive() {
            true => blockers.trailing_zeros(),
            false => 63 - blockers.leading_zeros(),
        } as usize;

        attacks |= Bitboard(RAY_ATTACKS[*i][square] ^ RAY_ATTACKS[*i][blocker]);
    }
    attacks
}

pub fn step_attacks(square: usize, deltas: &[i8]) -> Bitboard {
    let mut attacks = Bitboard(0);

    for delta in deltas {
        let next = square as i8 + *delta as i8;
        if next >= 0 && next < 64 {
            attacks |= Bitboard(1 << next);
        }
    }
    attacks
}
