use chessr::board::Board;

use anyhow::Result;

fn main() -> Result<()> {
    let board = Board::from_fen("rnbqkbnr/1ppp1ppp/8/4p3/p3P2P/P7/1PPP1PP1/RNBQKBNR w KQkq - 0 5")?;
    println!("{}", board);
    println!("{}", board.fen());

    Ok(())
}
