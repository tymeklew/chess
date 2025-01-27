use crate::game::Square;
use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

#[derive(Copy, Clone)]
pub struct BitBoard(pub u64);

impl BitBoard {
    ///* `shift` - How much to shift by
    ///* `positive` - Direction of shift positive for left shift and negative for right shift
    pub fn shift(self, shift: u64, positive: bool) -> BitBoard {
        match positive {
            true => self << shift,
            false => self.clone() >> shift,
        }
    }

    pub fn from_square(sqr: Square) -> BitBoard {
        let mut num: u64 = 0;
        num ^= 1_u64 << sqr.1 * 8 + sqr.0;
        BitBoard(num)
    }

    // Find the coordinates of all pieces on a bitboard
    pub fn all_coords(&self) -> Vec<Square> {
        let mut bb = self.clone();
        let mut coords: Vec<Square> = Vec::new();

        loop {
            let lsb: u64 = bb.0.trailing_zeros().into();
            if lsb >= 64 {
                break;
            }

            coords.push(Square::new(lsb as u8 % 8, lsb as u8 / 8));
            bb.0 ^= 1 << lsb;
        }
        coords
    }

    ///  parallel prefix-algorithm
    ///  Mirror a bitboard horizontaly around the center
    pub fn mirror_h(&self) -> BitBoard {
        let mut bb = self.clone();
        const k1: u64 = 0x5555555555555555;
        const k2: u64 = 0x3333333333333333;
        const k4: u64 = 0x0f0f0f0f0f0f0f0f;

        bb.0 = ((bb.0 >> 1) & k1) | ((bb.0 & k1) << 1);
        bb.0 = ((bb.0 >> 2) & k2) | ((bb.0 & k2) << 2);
        bb.0 = ((bb.0 >> 4) & k4) | ((bb.0 & k4) << 4);

        bb
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

impl Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl Shl<u64> for BitBoard {
    type Output = Self;
    fn shl(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl Shr<u64> for BitBoard {
    type Output = Self;
    fn shr(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}
