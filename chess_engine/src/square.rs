use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    // Horizontal
    pub rank: usize,
    // Vertical
    pub file: usize,
}

impl Square {
    pub fn new(rank: usize, file: usize) -> Square {
        Square { rank, file }
    }

    pub fn rank(&self) -> usize {
        self.rank
    }
    pub fn file(&self) -> usize {
        self.file
    }

    pub fn from_idx(idx: usize) -> Square {
        Square {
            rank: idx % 8,
            file: idx / 8,
        }
    }

    pub fn idx(&self) -> usize {
        self.file * 8 + self.rank
    }

    pub fn to_algebraic(&self) -> String {
        let file_char = (b'a' + self.rank as u8) as char;
        format!("{}{}", file_char, self.file)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (b'a' + self.rank as u8) as char, self.file + 1)
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
