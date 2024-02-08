use std::fmt::Display;

/// Represents a square on the board.
/// The first element represents the row and the second element the column.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Square(pub usize, pub usize);

impl Square {
    /// Tries to convert an algebraic notation string into a square
    pub fn from_algebraic_str(algebraic: &str) -> Option<Square> {
        let mut chars = algebraic.chars();
        let column_char = chars.next()?;
        let row_char = chars.next()?;

        if !('a'..='h').contains(&column_char) || !('1'..='8').contains(&row_char) {
            return None;
        }

        // 7 - () because the board is zero-indexed and the rows are reversed
        let row = 7 - (row_char as usize - 49);
        let column = column_char as usize - 97;

        Some((row, column).into())
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (row, column) = (self.0, self.1);

        let row_char = 8 - row;
        let column_char = column as u8 + 97;

        write!(f, "{}{}", column_char as char, row_char)
    }
}

impl From<(usize, usize)> for Square {
    fn from((row, col): (usize, usize)) -> Self {
        Square(row, col)
    }
}

impl PartialEq<(usize, usize)> for Square {
    fn eq(&self, (row, col): &(usize, usize)) -> bool {
        self.0 == *row && self.1 == *col
    }
}

impl std::ops::Add<(i8, i8)> for Square {
    type Output = Square;

    fn add(self, (row, col): (i8, i8)) -> Self::Output {
        Square((self.0 as i8 + row) as usize, (self.1 as i8 + col) as usize)
    }
}

impl std::ops::AddAssign<&(i8, i8)> for Square {
    fn add_assign(&mut self, (row, col): &(i8, i8)) {
        *self = *self + (*row, *col);
    }
}

impl std::ops::AddAssign<(i8, i8)> for Square {
    fn add_assign(&mut self, (row, col): (i8, i8)) {
        *self = *self + (row, col);
    }
}
