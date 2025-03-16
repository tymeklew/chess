use std::{fmt::Display, ops::Index};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Square {
    // Horizontal
    pub file: usize,
    // Vertical
    pub rank: usize,
}

impl Square {
    pub fn new(file: usize, rank: usize) -> Square {
        Square { file, rank }
    }

    pub fn rank(&self) -> usize {
        self.rank
    }
    pub fn file(&self) -> usize {
        self.file
    }

    pub fn from_idx(idx: usize) -> Square {
        Square {
            file: idx % 8,
            rank: idx / 8,
        }
    }

    pub fn idx(&self) -> usize {
        self.rank * 8 + self.file
    }

    pub fn to_algebraic(&self) -> String {
        let file_char = (b'a' + self.file as u8) as char;
        format!("{}{}", file_char, self.rank)
    }

    pub fn from_algebraic(input: String) -> Option<Square> {
        if input.len() != 2 {
            return None;
        }

        let mut chrs = input.chars();
        let file = chrs.nth(0)?;
        let rank = chrs.nth(0)?.to_digit(10)?;

        Some(Square {
            file: (file as u8 - b'a') as usize,
            rank: rank as usize - 1,
        })
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (b'a' + self.file as u8) as char, self.rank + 1)
    }
}

impl From<usize> for Square {
    fn from(value: usize) -> Self {
        Self {
            rank: value % 8,
            file: value / 8,
        }
    }
}

impl Into<usize> for Square {
    fn into(self) -> usize {
        self.file * 8 + self.rank
    }
}
