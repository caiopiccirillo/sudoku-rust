use std::collections::HashMap;

use super::sudoku::*;
#[derive(Debug)]
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

    pub fn check_board(&mut self) -> Result<(), ()> {
        self.check_rows()?;
        self.check_cols()?;
        self.check_subsquares()?;
        Ok(())
    }

    fn check_rows(&mut self) -> Result<(), ()> {
        let mut values = Vec::<Option<u8>>::new();
        let mut indexes = Vec::<String>::new();
        for row in self.data.rows.clone() {
            for col in self.data.cols.clone() {
                values.push(self.data.board[&format!("{}{}", row, col)]);
                indexes.push(format!("{}{}", row, col));
            }
            // Checking the row
            for i in 0..values.len() {
                for j in 0..values.len() {
                    match values[i] {
                        Some(v1) => match values[j] {
                            Some(v2) => {
                                if v1 == v2 && i != j {
                                    println!(
                                        "Error in row: {}, index {} equal to index {}",
                                        row, indexes[i], indexes[j]
                                    );
                                    return Err(());
                                }
                            }
                            // When the value is None of values[j], remove the possible number from list
                            None => {
                                match self.possible_numbers[&indexes[j]]
                                    .iter()
                                    .position(|i| *i == Some(v1))
                                {
                                    Some(index) => {
                                        self.possible_numbers
                                            .get_mut(&indexes[j])
                                            .unwrap()
                                            .remove(index);
                                    }
                                    None => {
                                        continue;
                                    }
                                }
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
            indexes.clear();
        }
        Ok(())
    }

    fn check_cols(&mut self) -> Result<(), ()> {
        let mut values = Vec::<Option<u8>>::new();
        let mut indexes = Vec::<String>::new();
        for col in self.data.cols.clone() {
            for row in self.data.rows.clone() {
                values.push(self.data.board[&format!("{}{}", row, col)]);
                indexes.push(format!("{}{}", row, col));
            }
            // Checking the col
            for i in 0..values.len() {
                for j in 0..values.len() {
                    match values[i] {
                        Some(v1) => match values[j] {
                            Some(v2) => {
                                if v1 == v2 && i != j {
                                    println!(
                                        "Error in col: {}, index {} equal to index {}",
                                        col, indexes[i], indexes[j]
                                    );
                                    return Err(());
                                }
                            }
                            // When the value is None of values[j], remove the possible number from list
                            None => {
                                match self.possible_numbers[&indexes[j]]
                                    .iter()
                                    .position(|i| *i == Some(v1))
                                {
                                    Some(index) => {
                                        self.possible_numbers
                                            .get_mut(&indexes[j])
                                            .unwrap()
                                            .remove(index);
                                    }
                                    None => {
                                        continue;
                                    }
                                }
                                // println!("{:?}", self.possible_numbers[&indexes[j]]);
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
            indexes.clear();
        }
        Ok(())
    }

    fn check_subsquares(&mut self) -> Result<(), ()> {
        let mut values = Vec::<Option<u8>>::new();
        let mut indexes = Vec::<String>::new();
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
                        indexes.push(format!(
                            "{}{}",
                            self.data.rows[i + row_offset],
                            self.data.cols[j + col_offset]
                        ));
                    }
                }
                // Checking the subsquare
                for i in 0..values.len() {
                    for j in 0..values.len() {
                        match values[i] {
                            Some(v1) => match values[j] {
                                Some(v2) => {
                                    if v1 == v2 && i != j {
                                        println!(
                                            "Error in subsquare: {}, index {} equal to index {}",
                                            subsquare, indexes[i], indexes[j]
                                        );
                                        return Err(());
                                    }
                                }
                                // When the value is None of values[j], remove the possible number from list
                                None => {
                                    match self.possible_numbers[&indexes[j]]
                                        .iter()
                                        .position(|i| *i == Some(v1))
                                    {
                                        Some(index) => {
                                            self.possible_numbers
                                                .get_mut(&indexes[j])
                                                .unwrap()
                                                .remove(index);
                                        }
                                        None => {
                                            continue;
                                        }
                                    }
                                    // println!("{:?}", self.possible_numbers[&indexes[j]]);
                                }
                            },
                            None => {
                                continue;
                            }
                        }
                    }
                }
                values.clear();
                indexes.clear();
                subsquare += 1;
                col_offset += 3;
            }
            col_offset = 0;
            row_offset += 3;
        }
        Ok(())
    }

    pub fn solve_board(&mut self) -> Sudoku {
        for _ in 0..81 {
            self.check_board().unwrap();
            for row in self.data.rows.clone() {
                for col in self.data.cols.clone() {
                    if self.possible_numbers[&format!("{}{}", row, col)].len() == 1 {
                        self.data.update_value_using_position(
                            format!("{}{}", row, col),
                            self.possible_numbers[&format!("{}{}", row, col)][0].unwrap(),
                        );
                    }
                }
            }
            println!("{}",self.data.num_empty_fields());
            if self.data.num_empty_fields() == 0 {
                break;
            }
        }
        self.data.clone()
    }
}
