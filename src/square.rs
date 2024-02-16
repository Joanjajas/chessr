use crate::board::BitBoard;

pub struct Square(pub u8);

impl Square {
    pub fn to_bb(&self) -> BitBoard {
        BitBoard(1 << self.0)
    }
}

impl std::ops::AddAssign for Square {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
