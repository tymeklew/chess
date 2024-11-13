use std::fmt::Display;

use crate::game::Square;

#[derive(Copy, Clone)]
pub struct BitBoard(pub u64);
impl BitBoard {
    pub fn from_square(sqr: Square) -> BitBoard {
        let mut num: u64 = 0;
        num ^= 1_u64 << sqr.1 * 8 + sqr.0;
        BitBoard(num)
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (63 - (rank * 8) - file);
                let chr = if self.0 & mask != 0 { '1' } else { '0' };
                write!(f, "{chr}")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}
/*let board = 268_435_456;
for rank in 0..8 {
    for file in (0..8).rev() {
        let mask = 1u64 << (63 - (rank * 8) - file);
    }
}
*/
