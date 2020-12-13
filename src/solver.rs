use std::iter::Enumerate;

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
        let mut values = Vec::<Option<u8>>::new();
        for row in self.data.rows.clone() {
            for col in self.data.cols.clone() {
                values.push(self.data.board[&format!("{}{}", row, col)]);
            }
            // Checking the row
            for i in 0..(values.len() - 1) {
                for j in (i + 1)..values.len() {
                    match values[i] {
                        Some(v1) => match values[j] {
                            Some(v2) => {
                                if v1 == v2 {
                                    println!(
                                        "Error in row: {}, index {} equal to index {}",
                                        row, i, j
                                    );
                                    return Err(());
                                }
                            }
                            None => {
                                continue;
                            }
                        },
                        None => {
                            continue;
                        }
                    }
                }
            }
            // Clearing the values to a new row
            values.clear();
        }
        Ok(())
    }

    fn check_cols(&self) -> Result<(), ()> {
        let mut values = Vec::<Option<u8>>::new();
        for col in self.data.cols.clone() {
            for row in self.data.rows.clone() {
                values.push(self.data.board[&format!("{}{}", row, col)]);
            }
            // Checking the col
            for i in 0..(values.len() - 1) {
                for j in (i + 1)..values.len() {
                    match values[i] {
                        Some(v1) => match values[j] {
                            Some(v2) => {
                                if v1 == v2 {
                                    println!(
                                        "Error in col: {}, index {} equal to index {}",
                                        col, i, j
                                    );
                                    return Err(());
                                }
                            }
                            None => {
                                continue;
                            }
                        },
                        None => {
                            continue;
                        }
                    }
                }
            }
            // Clearing the values to a new row
            values.clear();
        }
        Ok(())
    }

    fn check_subsquares(&self) -> Result<(), ()> {
        return Ok(());
    }
}
