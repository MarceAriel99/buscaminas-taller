pub mod board;
pub mod cell;

use board::Board;
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
    .expect("Should have been able to read the file")
}

fn main() {

    // Leo el archivo y lo transformo a un String
    let board_str = read_file("board.txt");
    // Creo un nuevo tablero a partir del archivo leido
    let mut board = Board::new(board_str).expect("Error");
    // Imprimo el tablero original
    println!("{}", board);
    // Calculo las bombas adyacentes a cada espacio vacío
    board.count_bombs();
    // Imprimo el tablero
    println!("{}", board);
}