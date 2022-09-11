use buscaminas::board::Board;

#[test]
fn board_initial_state() {
    let board_str = ".*.*.\n..*..\n..*..\n.....".to_string();
    let board = Board::new(board_str).unwrap();

    let expected_board = vec![".","*",".","*",".",".",".","*",".",".",".",".","*",".",".",".",".",".",".","."];

    assert_eq!(board.to_vec(), expected_board);
}

#[test]
fn board_counted_mines() {
    let board_str = ".*.*.\n..*..\n..*..\n.....".to_string();
    let mut board = Board::new(board_str).unwrap();
    board.count_mines();

    let expected_board = vec!["1","*","3","*","1","1","3","*","3","1",".","2","*","2",".",".","1","1","1","."];

    assert_eq!(board.to_vec(), expected_board);
}