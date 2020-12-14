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
            // Clearing the values to a new col
            values.clear();
        }
        Ok(())
    }

    fn check_subsquares(&self) -> Result<(), ()> {
        let mut values = Vec::<Option<u8>>::new();
        let mut subsquare = 0;
        let mut row_offset = 0;
        let mut col_offset = 0;
        // Changing offset in row
        for _ in 0..3 {
            // Changing offset in col
            for _ in 0..3 {
                for i in 0..3 {
                    for j in 0..3 {
                        values.push(
                            self.data.board[&format!(
                                "{}{}",
                                self.data.rows[i + row_offset],
                                self.data.cols[j + col_offset]
                            )],
                        );
                    }
                }
                // Checking the subsquare
                for i in 0..(values.len() - 1) {
                    for j in (i + 1)..values.len() {
                        match values[i] {
                            Some(v1) => match values[j] {
                                Some(v2) => {
                                    if v1 == v2 {
                                        println!(
                                            "Error in subsquare: {}, index {} equal to index {}",
                                            subsquare, i, j
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
                values.clear();
                subsquare += 1;
                col_offset += 3;
            }
            col_offset = 0;
            row_offset += 3;
        }
        Ok(())
    }
}
