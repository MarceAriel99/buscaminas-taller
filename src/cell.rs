/// Enumerates both types of cells in the board
pub enum Cell {
    /// Represents a mine
    Mine,
    /// Represents an empty space, with the amount of adjacent mines
    Empty(u8),
}
