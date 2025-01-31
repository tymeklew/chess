#[derive(Debug , Clone , Copy)]
pub struct Square {
    // Horizontal
    rank : usize,
    // Vertical
    file : usize
}

impl Square {
    pub fn new(rank : usize, file : usize) -> Square {
        Square { rank, file }
    }

    pub fn rank(&self) -> usize { self.rank }
    pub fn file(&self) -> usize { self.file }

    pub fn idx(&self) -> usize { self.file * 8 + self.rank }

    pub fn to_algebraic(&self) -> String {
        let file_char = ("a".."b")[self.rank];
        format!("{}{}" , file_char , self.file)
    }
}

impl From<usize> for Square {
    fn from(value: usize) -> Self {
        Self {
            rank : value % 8,
            file : value / 8
        }
    }
}

impl Into<usize> for Square {
    fn into(self) -> usize {
        self.file * 8 + self.rank
    }
}