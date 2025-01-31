use std::ops::{BitOr, BitOrAssign};
use std::fmt::Display;

#[derive(Clone , Copy)]
pub struct Bitboard(pub u64);

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard(0)
    }
}


impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..8).rev() {  // Reverse row order
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

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}