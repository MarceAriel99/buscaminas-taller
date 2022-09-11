use crate::cell::Cell;
use std::fmt;

pub struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Board {

    pub fn new(board_str: String) -> Result<Self, String> {

        let lines: Vec<&str> = board_str.split('\n').collect();
        let mut grid: Vec<Vec<Cell>> = Vec::new();

        let line_length = lines[0].len();

        for line in lines{
            if line.is_empty() {continue}
            if line.len() != line_length{return Err("All lines must have the same length".to_string())}

            let mut gridline: Vec<Cell> = Vec::new();

            for character in line.as_bytes(){
                if *character != 46 && *character != 42 {
                    return Err("Wrong character in file. Expected only '*' or '.'".to_string());
                }
                //Add Cell to grid according to character
                let element = match character {
                    46 => Cell::Empty(0),
                    42 => Cell::Mine,
                    _ => unreachable!(), // Already accounted for and returned Error
                };
                gridline.push(element);
            }
            grid.push(gridline);
        }
        Ok(Self{grid})
    }

    pub fn count_mines(&mut self) {

        let mut new_grid: Vec<Vec<Cell>> = Vec::new();

        for (y, row) in self.grid.iter().enumerate() {
            let mut new_gridline: Vec<Cell> = Vec::new();
            for (x, cell) in row.iter().enumerate() {
                let new_cell = match cell {
                    Cell::Mine => Cell::Mine,
                    Cell::Empty(_) => Cell::Empty(self.calculate_near_mines(x, y)),
                };
                new_gridline.push(new_cell);
            }
            new_grid.push(new_gridline);
        }
        self.grid = new_grid;
    }

    fn calculate_near_mines(&self, x: usize, y: usize) -> u8 {

        let mut count: u8 = 0;

        for offset_x in 0..3 {
            for offset_y in 0..3 {
                // If coordinate will be negative, continue. As usize can't be negative.
                if x == 0 && offset_x == 0 || y == 0 && offset_y == 0 {continue;}

                if let Some(row) = self.grid.get(y + offset_y - 1) {
                    // There is a valid row
                    if let Some(cell) = row.get(x + offset_x - 1) {
                        // There is a valid cell
                        match cell {
                            Cell::Mine => count += 1,
                            Cell::Empty(_) => (),
                        }
                    }
                }
            }
        }
        count
    }

    pub fn to_vec(&self) -> Vec<String> {
        let mut vect: Vec<String> = vec![];
        for row in &self.grid {
            for cell in row {
                let character = match cell {
                    Cell::Mine => "*".to_string(),
                    Cell::Empty(0) => ".".to_string(),
                    Cell::Empty(num) => num.to_string(),
                };
                vect.push(character);
            }
        }
        vect
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                let character = match cell {
                    Cell::Mine => "*".to_string(),
                    Cell::Empty(0) => ".".to_string(),
                    Cell::Empty(num) => num.to_string(),
                };
                write!(f, "{}", character)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::Board;

    #[test]
    fn create_board() {
        let board_str = "*..*\n.**.\n*.*.\n.*.*".to_string();
        let _board = Board::new(board_str).unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_to_create_board_wrong_characters() {
        let board_str = "not\nthe\ncorrect\nformat".to_string();
        let _board = Board::new(board_str).unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_to_create_board_different_line_lengths() {
        let board_str = "*..*..\n.**.*\n*.*.\n.*.**.".to_string();
        let _board = Board::new(board_str).unwrap();
    }

    #[test]
    fn count_adjacent_mines_to_cell() {
        let board_str = "*..*\n.**.\n*.*.\n.*.*".to_string();
        let board = Board::new(board_str).unwrap();
        assert_eq!(board.calculate_near_mines(1, 2), 5);
    }

}