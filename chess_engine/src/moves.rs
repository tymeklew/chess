use crate::{board::Board, pieces::Pieces, square::Square};
use std::fmt::Display;

pub trait Move : Display {
    fn apply(&self , board : &mut Board);
    fn undo(&self) -> Box<dyn Move>;
    fn capture(&self) -> Option<Pieces>;
}

pub struct BasicMove {
    from : Square,
    to : Square,
}
impl Display for BasicMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

impl Move for BasicMove {
    fn apply(&self , board : &mut Board) {
        let from_side = board.get_side(self.from);
        let to_side = board.get_side(self.to);

        board.sides[from_side].0 ^= 1 << self.from.idx();
        board.sides[from_side].0
    }
    fn undo(&self) -> Box<dyn Move> {
        todo!()
    }
    fn capture(&self) -> Option<Pieces> {
        None
    }
}

impl BasicMove {
    pub fn new(from : Square , to : Square) -> Self {
        BasicMove {
            from,
            to,
        }
    }
}

pub struct Capture {
    from : Square,
    to : Square,
    capture : Pieces,
}

impl Display for Capture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} capturing {}", self.from, self.to, self.capture)
    }
}

impl Move for Capture {
    fn apply(&self , board : &mut Board) {
        todo!()
    }

    fn undo(&self) -> Box<dyn Move> {
       todo!() 
    }

    fn capture(&self) -> Option<Pieces> {
        Some(self.capture)
    }
}

impl Capture {
    pub fn new(from : Square , to : Square , capture : Pieces) -> Self {
        Capture {
            from,
            to,
            capture,
        }
    }
}