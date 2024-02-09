use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};

use anyhow::Result;
use chessr::Board;
use rand::random;

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    if let Err(e) = run() {
        println!("[App Error]: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut input = String::new();
    print!("Select a mode (fen, rand, rep, new): ");
    stdout().flush()?;
    stdin().read_line(&mut input)?;

    match input.trim() {
        "fen" => {
            let mut fen = String::new();
            print!("Enter FEN: ");
            stdout().flush()?;
            stdin().read_line(&mut fen)?;

            play(&fen)?;
            Ok(())
        }
        "new" => {
            play(STARTPOS)?;
            Ok(())
        }
        "rep" => parse_lichess_moves(),
        "rand" => random_game(),
        _ => Ok(()),
    }
}

fn play(startpos: &str) -> Result<()> {
    let mut board = Board::from_fen(startpos)?;
    println!("");
    println!("============================================================");
    println!("");
    println!("{}", board);
    println!("");
    println!("FEN: {}", board.fen());
    println!("");

    loop {
        if board.checkmate() {
            println!("Checkmate");
            break;
        } else if board.draw() {
            println!("Draw");
            break;
        }

        let mut r#move = String::new();
        print!("Play Move ({}): {}", board.active_color, r#move);
        stdout().flush()?;
        stdin().read_line(&mut r#move)?;
        let made_move = board.make_move(r#move.trim());
        if made_move.is_none() {
            continue;
        }

        println!("");
        println!("============================================================");
        println!("");
        println!("{}", board);
        println!("");
        println!("FEN: {}", board.fen());
        println!("");
        print!("Last Move ({}): {}", board.active_color.invert(), r#move);
    }

    Ok(())
}

fn random_game() -> Result<()> {
    let mut board = Board::new();
    println!("");
    println!("============================================================");
    println!("");
    println!("{}", board);
    println!("");
    println!("FEN: {}", board.fen());
    println!("");

    loop {
        if board.checkmate() {
            println!("Checkmate");
            break;
        } else if board.draw() {
            println!("Draw");
            break;
        }

        let legal_moves = board.legal_moves();
        let r#move = legal_moves[random::<usize>() % legal_moves.len()];
        println!(
            "Play Move ({}): {}",
            board.active_color,
            r#move.to_uci_str()
        );
        board.make_move(&r#move.to_uci_str());

        println!("");
        println!("============================================================");
        println!("");
        println!("{}", board);
        println!("");
        println!("FEN: {}", board.fen());
        println!("");
        println!(
            "Last Move ({}): {}",
            board.active_color.invert(),
            r#move.to_uci_str()
        );
    }
    Ok(())
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

    let mut board = Board::new();
    let mut sum = 0;

    println!("");
    println!("============================================================");
    println!("");
    println!("{}", board);
    println!("");
    println!("FEN: {}", board.fen());
    println!("");

    moves.iter().skip(1).for_each(|w| {
        if sum == 2 {
            sum = 0;
            return;
        }
        println!("Play Move ({}): {}", board.active_color, w);
        board.make_move(w);

        println!("");
        println!("============================================================");
        println!("");
        println!("{}", board);
        println!("");
        println!("FEN: {}", board.fen());
        println!("");
        println!("Last Move ({}): {}", board.active_color.invert(), w);
        sum += 1;
    });
    Ok(())
}
