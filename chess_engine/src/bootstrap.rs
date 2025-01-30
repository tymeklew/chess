use crate::game::FULL_COL;

// Deltas 
pub const WHITE_PAWN_DELTAS : [i8 ; 2] = [7 , 9];
pub const BLACK_PAWN_DELTAS : [i8 ; 2] = [-7 , -9];
pub const ROOK_DELTAS : [i8 ; 4] = [8 , 1 , -8 , -1];
pub const BISHOP_DELTAS :[i8 ; 4] =  [7 , 9 , -7 , - 9];
pub const QUEEN_DELTAS : [i8 ; 8] = [8 , 1 , -8 , -1 , 7 , 9 , -7 , - 9];
pub const KING_DELTAS : [i8 ; 8] = [7 , 8 , 9 , -1 , 1 , -9 , - 8 , -7];

const fn init_attacks(deltas : &[i8] ) -> [u64;64]{
    let mut table = [0;64];
    let mut i = 0;
    while i < 63 {
        table[i] = sliding_attacks(i as i8, deltas);
        i += 1;
    }

    table
}

const fn sliding_attacks(square : i8 , deltas : &[i8]) -> u64 {
    let mut attack : u64 = 0;

    let mut i = 0;
    let len = deltas.len();

    while i < len {
        let mut prev = square;
        loop {
            let sq   = match prev.checked_add(deltas[i]) {
                Some(n) if n <= 63 && n >= 0 => n,
                _ => break, 
            };

            let file_diff = (sq & 0x7) - (prev & 0x7); 
            if file_diff > 2 || file_diff < -2 {
                break;
            }

            attack |=  (1 as u64) << sq;
            prev = sq;
        }
        i += 1;
    }

    attack
}

pub const ROOK_ATTACKS : [u64 ; 64] = init_attacks(&ROOK_DELTAS);
pub const BISHOP_ATTACKS : [u64 ; 64] = init_attacks(&BISHOP_DELTAS);
pub const QUEEN_ATTACKS : [u64 ; 64] = init_attacks(&QUEEN_DELTAS);