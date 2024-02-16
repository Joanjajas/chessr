use anyhow::Result;
use chessr::board::Board;
use chessr::consts::*;
use std::time::Instant;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let start = Instant::now();
    let board = Board::from_fen("rnbqkbnr/1p1ppppp/p7/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 3")?;
    println!("Elapsed time: {:.2?}", start.elapsed());
    println!("{}", board);

    Ok(())
}
