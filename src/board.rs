use crate::cell::Cell;
use std::fmt;

pub struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Board {

    pub fn new(board_str: String) -> Result<Self, String> {

        let lines: Vec<&str> = board_str.split("\n").collect();
        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for line in lines{
            let mut gridline: Vec<Cell> = Vec::new();
            for character in line.as_bytes(){
                if *character != 46 && *character != 42 {
                    return Err("Caracter incorrecto en el archivo".to_string());
                }
                //Add bomb if bomb, add empty if empty
                let element = match character {
                    46 => Cell::Empty(0),
                    42 => Cell::Bomb,
                    _ => unreachable!(), // Already accounted for and returned Error
                };
                gridline.push(element);
            }
            grid.push(gridline);
        }
        Ok(Self{grid})
    }

    pub fn count_bombs(&mut self) {

        let mut new_grid: Vec<Vec<Cell>> = Vec::new();

        for (y, row) in self.grid.iter().enumerate() {
            let mut new_gridline: Vec<Cell> = Vec::new();
            for (x, cell) in row.iter().enumerate() {
                let new_cell = match cell {
                    Cell::Bomb => Cell::Bomb,
                    Cell::Empty(_) => Cell::Empty(self.calculate_near_bombs(x, y)),
                };
                new_gridline.push(new_cell);
            }
            new_grid.push(new_gridline);
        }
        self.grid = new_grid;
    }

    fn calculate_near_bombs(&self, x: usize, y: usize) -> u8 {

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
                            Cell::Bomb => count += 1,
                            Cell::Empty(_) => (),
                        }
                    }
                }
            }
        }
        return count;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                let character = match cell {
                    Cell::Bomb => "*".to_string(),
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