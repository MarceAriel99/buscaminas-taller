pub mod board;
pub mod cell;

use board::Board;
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
    .expect("Should have been able to read the file")
}

fn main() {
    // Reads file and returns string
    let board_str = read_file("board.txt");
    // Create new board using string
    let mut board = Board::new(board_str).expect("Error");
    // Display original board
    println!("{}", board);
    // Calculate adjacent mines to each empty space
    board.count_mines();
    // Display updated board
    println!("{}", board);
}