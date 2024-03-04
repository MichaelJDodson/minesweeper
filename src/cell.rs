use std::fmt::Display;

use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
/// contains the information pertaining to a cell
pub struct Cell {
    is_mine: bool,
    /// number of mines surrounding this cell
    local_mines: usize,
    /// determines if the user has picked to reveal contents of cell
    is_revealed: bool,
}
impl Display for Cell {
    /// display as mine as "*". If empty, displays the number of mines in cells around it
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_mine {
            write!(f, "*")?;
        }else {
            write!(f, "{}", self.local_mines)?;
        }
        return Ok(());
    }
}
impl Cell {
    /// a clear cell used for a default value
    pub const CLEAR: Self = Self {
        is_mine: false,
        local_mines: 0,
        is_revealed: false,
    };
    /// used to generate a random cell
    pub fn random() -> Self {
        let mut rng = thread_rng(); // random thread value
        return Self { is_mine: rng.gen_bool(0.5), local_mines: 0, is_revealed: false };
    }
    pub fn is_mine(&self) -> bool {
        return self.is_mine;
    }
    pub fn is_revealed(&self) -> bool {
        return self.is_revealed;
    }
    pub fn local_mines(&self) -> usize {
        return self.local_mines;
    }
    pub fn local_mines_set(&mut self, local_mines: usize) {
        self.local_mines = local_mines;
    } 
}