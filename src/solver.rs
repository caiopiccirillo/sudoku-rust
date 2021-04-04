use std::collections::HashMap;
use std::time::Instant;

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
                        Some(v1) => {
                            match self.possible_numbers[&indexes[i]]
                                .iter()
                                .position(|i| *i != Some(v1))
                            {
                                Some(index) => {
                                    self.possible_numbers
                                        .get_mut(&indexes[i])
                                        .unwrap()
                                        .remove(index);
                                }
                                None => {}
                            }
                            match values[j] {
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
                            }
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
            self.unique_possibility(&mut indexes).unwrap();
            self.naked_twin_strategy(&mut indexes).unwrap();
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
                        Some(v1) => {
                            match self.possible_numbers[&indexes[i]]
                                .iter()
                                .position(|i| *i != Some(v1))
                            {
                                Some(index) => {
                                    self.possible_numbers
                                        .get_mut(&indexes[i])
                                        .unwrap()
                                        .remove(index);
                                }
                                None => {}
                            }
                            match values[j] {
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
                            }
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
            self.unique_possibility(&mut indexes).unwrap();
            self.naked_twin_strategy(&mut indexes).unwrap();
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
                self.unique_possibility(&mut indexes).unwrap();
                self.naked_twin_strategy(&mut indexes).unwrap();
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

    fn unique_possibility(&mut self, list_index_by_group: &mut Vec<String>) -> Result<(), ()> {
        if list_index_by_group.len() == 0 {
            return Err(());
        }
        for index1 in list_index_by_group.clone() {
            if self.possible_numbers[&index1].len() >= 2 {
                for value1 in self.possible_numbers[&index1].clone() {
                    let mut unique = true;
                    for index2 in list_index_by_group.clone() {
                        if index1 != index2 {
                            for value2 in self.possible_numbers[&index2].clone() {
                                if value1.unwrap() == value2.unwrap() {
                                    unique = false;
                                    break;
                                }
                            }
                        }
                    }
                    if unique {
                        println!(
                            "Found unique: {} {:?}",
                            index1, self.possible_numbers[&index1]
                        );
                        match self.possible_numbers[&index1]
                            .iter()
                            .position(|i| *i != value1)
                        {
                            Some(index) => {
                                self.possible_numbers
                                    .get_mut(&index1)
                                    .unwrap()
                                    .remove(index);
                            }
                            None => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn naked_twin_strategy(&mut self, list_index_by_group: &mut Vec<String>) -> Result<(), ()> {
        if list_index_by_group.len() == 0 {
            return Err(());
        }
        let mut first_twin_index = String::new();
        let mut second_twin_index = String::new();
        for index1 in 0..list_index_by_group.len() {
            if self.possible_numbers[&list_index_by_group[index1]].len() == 2 {
                for index2 in index1..list_index_by_group.len() {
                    if self.possible_numbers[&list_index_by_group[index2]].len() == 2
                        && self.possible_numbers[&list_index_by_group[index1]]
                            == self.possible_numbers[&list_index_by_group[index2]]
                        && index1 != index2
                    {
                        if first_twin_index.is_empty() && second_twin_index.is_empty() {
                            first_twin_index = list_index_by_group[index1].clone();
                            second_twin_index = list_index_by_group[index2].clone();
                        }
                        // println!(
                        //     "Found {} {:?} equal to {} {:?}",
                        //     list_index_by_group[index1],
                        //     self.possible_numbers[&list_index_by_group[index1]],
                        //     list_index_by_group[index2],
                        //     self.possible_numbers[&list_index_by_group[index2]]
                        // );
                    }
                }
            }
        }
        if !first_twin_index.is_empty() && !second_twin_index.is_empty() {
            for index in list_index_by_group.clone() {
                if index != first_twin_index && index != second_twin_index {
                    match self.possible_numbers[&index].iter().position(|i| {
                        *i == self.possible_numbers[&first_twin_index][0]
                            || *i == self.possible_numbers[&first_twin_index][1]
                    }) {
                        Some(i) => {
                            self.possible_numbers.get_mut(&index).unwrap().remove(i);
                            // println!(
                            //     "Removing {} {:?}",
                            //     index,
                            //     self.possible_numbers[&index]
                            // );
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn solve_board(&mut self) -> Result<Sudoku, &str> {
        let start = Instant::now();
        let mut epoch = 1;
        loop {
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
            // println!("{}", self.data.num_empty_fields());
            if self.data.num_empty_fields() == 0 {
                break;
            }
            if start.elapsed().as_millis() >= 100 {
                break;
            }
            println!("{}", epoch);
            epoch += 1;
        }
        for row in self.data.rows.clone() {
            for col in self.data.cols.clone() {
                println!(
                    "{}{} {:?}",
                    row,
                    col,
                    self.possible_numbers[&format!("{}{}", row, col)]
                );
            }
        }
        if start.elapsed().as_millis() >= 100 {
            println!("Time elapsed in solve_board() is: {:?}", start.elapsed());
            println!("{:?}", self.data);
            return Err("Timeout error");
        }
        Ok(self.data.clone())
    }
}
