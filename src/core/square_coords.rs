use std::fmt::Display;

/// Represents a square on the board.
/// The first element represents the row and the second element the column.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SquareCoords(pub usize, pub usize);

impl SquareCoords {
    /// Tries to convert an algebraic notation string into a square
    pub fn from_san_str(algebraic: &str) -> Option<SquareCoords> {
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

    /// Returns true if the square coordinates form part of the board
    pub fn inside_board(&self) -> bool {
        (0..=7).contains(&self.0) && (0..=7).contains(&self.1)
    }
}

impl Display for SquareCoords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (row, column) = (self.0, self.1);

        let row_char = 8 - row;
        let column_char = column as u8 + 97;

        write!(f, "{}{}", column_char as char, row_char)
    }
}

impl From<(usize, usize)> for SquareCoords {
    fn from((row, col): (usize, usize)) -> Self {
        SquareCoords(row, col)
    }
}

impl PartialEq<(usize, usize)> for SquareCoords {
    fn eq(&self, (row, col): &(usize, usize)) -> bool {
        self.0 == *row && self.1 == *col
    }
}

impl std::ops::Add<(i8, i8)> for SquareCoords {
    type Output = SquareCoords;

    fn add(self, (row, col): (i8, i8)) -> Self::Output {
        SquareCoords((self.0 as i8 + row) as usize, (self.1 as i8 + col) as usize)
    }
}

impl std::ops::Add<&(i8, i8)> for SquareCoords {
    type Output = SquareCoords;

    fn add(self, (row, col): &(i8, i8)) -> Self::Output {
        SquareCoords((self.0 as i8 + row) as usize, (self.1 as i8 + col) as usize)
    }
}

impl std::ops::Sub<(i8, i8)> for SquareCoords {
    type Output = SquareCoords;

    fn sub(self, (row, col): (i8, i8)) -> Self::Output {
        SquareCoords((self.0 as i8 - row) as usize, (self.1 as i8 - col) as usize)
    }
}

impl std::ops::Sub<&(i8, i8)> for SquareCoords {
    type Output = SquareCoords;

    fn sub(self, (row, col): &(i8, i8)) -> Self::Output {
        SquareCoords((self.0 as i8 - row) as usize, (self.1 as i8 - col) as usize)
    }
}

impl std::ops::AddAssign<&(i8, i8)> for SquareCoords {
    fn add_assign(&mut self, (row, col): &(i8, i8)) {
        *self = *self + (*row, *col);
    }
}

impl std::ops::AddAssign<(i8, i8)> for SquareCoords {
    fn add_assign(&mut self, (row, col): (i8, i8)) {
        *self = *self + (row, col);
    }
}
