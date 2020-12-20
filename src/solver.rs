use std::collections::HashMap;

use super::sudoku::*;
pub struct Solver {
    pub data: Sudoku,
    possible_numbers: HashMap<String, Vec<Option<u8>>>,
}

impl Solver {
    pub fn new(data: Sudoku) -> Self {
        let mut possible_numbers: HashMap<String, Vec<Option<u8>>> = HashMap::new();
        let rows = (b'a'..=b'i') // Start as u8
            .map(|c| c as char) // Convert all to chars
            .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
            .collect::<Vec<_>>(); // Collect as Vec<char>
        let cols = (b'1'..=b'9') // Start as u8
            .map(|c| c as char) // Convert all to chars
            .filter(|c| c.is_numeric()) // Filter only numeric chars
            .collect::<Vec<_>>(); // Collect as Vec<char>
        let possibilities = (1..=9)
            .map(|a| (Some(a))) // Convert all to Option
            .collect::<Vec<Option<u8>>>(); // Collect as Vec<Option<u8>>
        for r in rows.iter() {
            for c in cols.iter() {
                // Insert vector with all possibilities considering blank board
                possible_numbers.insert(format!("{}{}", r, c), possibilities.clone());
            }
        }
        Self {
            data,
            possible_numbers,
        }
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
