use super::sudoku::*;
pub struct Solver {
    pub data: Sudoku,
}

impl Solver {
    pub fn new(data: Sudoku) -> Self {
        Self { data }
    }

    pub fn check_board(&self) -> Result<(), ()> {
        self.check_rows()?;
        self.check_cols()?;
        self.check_subsquares()?;
        Ok(())
    }

    fn check_rows(&self) -> Result<(), ()> {
        return Ok(());
    }

    fn check_cols(&self) -> Result<(), ()> {
        return Ok(());
    }

    fn check_subsquares(&self) -> Result<(), ()> {
        return Ok(());
    }
}
