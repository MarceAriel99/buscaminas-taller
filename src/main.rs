use buscaminas::board::Board;
use std::{fs, io};

pub fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn main() {
    
    let path = "board.txt";

    // Reads file and returns string
    let board_str = match read_file(path) {
        Ok(str) => str,
        Err(error) => {println!("Error reading file: {}: {}", error, path); return},
    };

    // Create new board using string
    let mut board: Board = match Board::new(board_str) {
        Ok(res) => res,
        Err(msg) => {println!("Error converting file to board: {}", msg); return}, 
    };
    
    // Display original board
    println!("{}", board);
    // Calculate adjacent mines to each empty space
    board.count_mines();
    // Display updated board
    println!("{}", board);
}