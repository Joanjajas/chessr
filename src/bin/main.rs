use anyhow::Result;
use std::fs::read_to_string;

use chessr::board;

fn main() {
    if let Err(e) = run() {
        println!("[App Error]: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("FEN, rep or new:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "fen" => {
            println!("Enter FEN:");
            let mut fen = String::new();
            std::io::stdin().read_line(&mut fen)?;
            let board = board::Board::from_fen(&fen)?;

            play(board)?;
            Ok(())
        }
        "new" => {
            let board = board::Board::new();
            play(board)?;
            Ok(())
        }
        "rep" => parse_lichess_moves(),
        _ => Ok(()),
    }
}

fn play(mut board: board::Board) -> Result<()> {
    loop {
        println!("====================");
        println!("{}", board);
        let mut r#move = String::new();
        std::io::stdin().read_line(&mut r#move)?;
        board.make_move_algebraic(r#move.trim());
        println!("{}", board.fen());
    }
}

fn parse_lichess_moves() -> Result<()> {
    let re = regex::Regex::new(r"(\{[^}]+\}|\([^)]+\)|\[[^)]+\])").unwrap();
    let re2 = regex::Regex::new(r"(\d+)(\.{3})").unwrap();
    let re3 = regex::Regex::new(r"[!#?+]").unwrap();
    let moves = read_to_string("moves.txt")?;
    let moves = re.replace_all(&moves, "");
    let moves = re2.replace_all(&moves, "");
    let moves = re3.replace_all(&moves, "");
    let moves = moves.split_whitespace().collect::<Vec<_>>();

    let mut board = board::Board::new();
    let mut sum = 0;
    moves.iter().skip(1).for_each(|w| {
        if sum == 2 {
            sum = 0;
            return;
        }
        board.make_move_algebraic(w);
        println!("====================");
        println!("{}", board);
        println!("{}", w);
        println!("{}", board.fen());
        sum += 1;
    });
    Ok(())
}
