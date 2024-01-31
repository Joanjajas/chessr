/// Tries to convert an algebraic notation string into a tuple of coordinates
pub fn algebraic_to_coordinates(algebraic: &str) -> Option<(usize, usize)> {
    let mut chars = algebraic.chars();
    let column_char = chars.next()?;
    let row_char = chars.next()?;

    if !('a'..='h').contains(&column_char) || !('1'..='8').contains(&row_char) {
        return None;
    }

    // 7 - () because the board is zero-indexed and the rows are reversed
    let row = 7 - (row_char as usize - 49);
    let column = column_char as usize - 97;

    Some((row, column))
}

/// Tries to convert a tuple of coordinates into an algebraic notation string
pub fn coordinates_to_algebraic(coordinates: (usize, usize)) -> Option<String> {
    let (row, column) = coordinates;

    if !(0..=7).contains(&row) || !(0..=7).contains(&column) {
        return None;
    }

    let row_char = 8 - row;
    let column_char = column as u8 + 97;

    let algebraic = format!("{}{}", column_char as char, row_char);
    Some(algebraic)
}
