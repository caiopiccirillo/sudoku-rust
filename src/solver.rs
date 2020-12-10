use super::sudoku::*;
pub struct Solver {
    pub data: Sudoku,
}
impl Solver {
    pub fn new(data: Sudoku) -> Self {
        Self { data }
    }

    pub fn check_board(&self) -> Result<(), ()> {
        // self.data.board;
        return Ok(());
    }
}
