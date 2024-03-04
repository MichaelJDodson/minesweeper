use std::fmt::Display;

use crate::cell::Cell;


/// Board contains an 2D array of cells
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    pub cells: [[Cell; WIDTH]; HEIGHT],
}
impl<const W: usize, const H: usize> Display for Board<W, H> {
    /// displays a board
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &row in self.cells.iter() {
            for &element in row.iter() {
                write!(f, "{} ", element)?;
            }
            write!(f, "\n")?;
        }
        return Ok(());
    }
}
impl<const W: usize, const H: usize> Board<W, H> {
    /// Initialize the minesweeper board with random true/false
    pub fn initialize_random() -> Self {
        let mut cells = [[Cell::CLEAR; W] ; H];
        
        for row in cells.iter_mut() {
            for element in row.iter_mut() {
                *element = Cell::random();
            }
        }
        // place the array of cells in the board
        let mut board = Self {
            cells: cells,
        };
        // determine the local mine count
        board.local_mine_count();

        return board;
    }
    /// determine the local mine count for each cell of the board and assigns it
    pub fn local_mine_count(&mut self) {
        let mut local_mine_count: usize = 0;


        // I didn't use an iterator bc I couldn't wrap my head around the logic for this given scenario...
        // It needed to be able to iterate across local cells within the board while also iterating across the entire board
        for row in 0..W {
            for col in 0..H {
                for i in (row as isize - 1)..=(row as isize + 1) {
                    for j in (col as isize - 1)..=(col as isize + 1) {
                        // Check if the neighboring cells are within bounds
                        if i >= 0 && j >= 0 && i < self.cells.len() as isize && j < self.cells[i as usize].len() as isize {
                            // Increment the local mine count
                            if self.cells[i as usize][j as usize].is_mine() {
                                local_mine_count += 1;
                            }
                        }
                    }
                }

                // if the current cell has a mine it is subtracted from the local mine count
                if self.cells[row][col].is_mine() {
                    local_mine_count -= 1;
                }

                // set the local mine count for the given cell, based on the count accumulated
                self.cells[row][col].local_mines_set(local_mine_count);
                // reset the local mine count for the next cell in the iteration
                local_mine_count = 0;

            }
        }
    }
}
