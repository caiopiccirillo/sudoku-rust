use std::collections::HashMap;
use std::time::Instant;

use super::sudoku::*;

const TIMEOUT: u128 = 150;

#[derive(Debug)]
pub struct Solver {
    pub data: Sudoku,
    possible_numbers: HashMap<String, Vec<Option<u8>>>,
    instant: Instant,
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
        let instant = Instant::now();
        Self {
            data,
            possible_numbers,
            instant,
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
                        // println!(
                        //     "Found unique: {} {:?}",
                        //     index1, self.possible_numbers[&index1]
                        // );
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
        if self.instant.elapsed().as_millis() >= TIMEOUT {
            // println!(
            //     "Time elapsed in solve_board() is: {:?}",
            //     self.instant.elapsed()
            // );
            return Err("Timeout error");
        }
        let mut previous_num_empty_fields = self.data.num_empty_fields();
        if previous_num_empty_fields == 81 {
            return Err("Empty sudoku board");
        }
        let mut epoch = 1;
        loop {
            match self.check_board() {
                Ok(_) => {}
                Err(_) => {
                    return Err("Checking board error");
                }
            }
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
            if previous_num_empty_fields == self.data.num_empty_fields() {
                let data_clone = self.data.clone();
                let possible_numbers_clone = self.possible_numbers.clone();
                'outer: for row in self.data.rows.clone() {
                    for col in self.data.cols.clone() {
                        if self.possible_numbers[&format!("{}{}", row, col)].len() == 2 {
                            self.data.update_value_using_position(
                                format!("{}{}", row, col),
                                self.possible_numbers[&format!("{}{}", row, col)][0].unwrap(),
                            );
                            // println!("Trying {}{}",row,col);
                            break 'outer;
                        }
                    }
                }
                match self.solve_board() {
                    Ok(board) => {
                        return Ok(board);
                    }
                    Err(_) => {
                        self.possible_numbers = possible_numbers_clone;
                        self.data = data_clone;
                        'outer_: for row in self.data.rows.clone() {
                            for col in self.data.cols.clone() {
                                if self.possible_numbers[&format!("{}{}", row, col)].len() == 2 {
                                    self.data.update_value_using_position(
                                        format!("{}{}", row, col),
                                        self.possible_numbers[&format!("{}{}", row, col)][1]
                                            .unwrap(),
                                    );
                                    // println!("Trying {}{}",row,col);
                                    break 'outer_;
                                }
                            }
                        }
                    }
                }
            }
            if self.data.num_empty_fields() == 0 {
                break;
            }
            if self.instant.elapsed().as_millis() >= TIMEOUT {
                break;
            }
            println!("Epoch {}", epoch);
            epoch += 1;
            previous_num_empty_fields = self.data.num_empty_fields();
        }
        if self.instant.elapsed().as_millis() >= TIMEOUT {
            // println!(
            //     "Time elapsed in solve_board() is: {:?}",
            //     self.instant.elapsed()
            // );
            return Err("Timeout error");
        }
        Ok(self.data.clone())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sample_very_hard() -> Sudoku {
        let mut sample_board = Sudoku::new();
        sample_board.update_value_using_position("a6".to_string(), 9);
        sample_board.update_value_using_position("a8".to_string(), 7);
        sample_board.update_value_using_position("b5".to_string(), 8);
        sample_board.update_value_using_position("b6".to_string(), 2);
        sample_board.update_value_using_position("b8".to_string(), 5);
        sample_board.update_value_using_position("c1".to_string(), 3);
        sample_board.update_value_using_position("c2".to_string(), 2);
        sample_board.update_value_using_position("c3".to_string(), 7);
        sample_board.update_value_using_position("c8".to_string(), 4);
        sample_board.update_value_using_position("d2".to_string(), 1);
        sample_board.update_value_using_position("d3".to_string(), 6);
        sample_board.update_value_using_position("d5".to_string(), 4);
        sample_board.update_value_using_position("e2".to_string(), 5);
        sample_board.update_value_using_position("e7".to_string(), 3);
        sample_board.update_value_using_position("f5".to_string(), 9);
        sample_board.update_value_using_position("f7".to_string(), 7);
        sample_board.update_value_using_position("g4".to_string(), 6);
        sample_board.update_value_using_position("g9".to_string(), 5);
        sample_board.update_value_using_position("h1".to_string(), 8);
        sample_board.update_value_using_position("h3".to_string(), 2);
        sample_board.update_value_using_position("i3".to_string(), 4);
        sample_board.update_value_using_position("i4".to_string(), 2);
        sample_board.update_value_using_position("i9".to_string(), 8);
        return sample_board;
    }

    fn generate_sample_hard() -> Sudoku {
        let mut sample_board = Sudoku::new();
        sample_board.update_value_using_position("a1".to_string(), 8);
        sample_board.update_value_using_position("a4".to_string(), 4);
        sample_board.update_value_using_position("a6".to_string(), 6);
        sample_board.update_value_using_position("a9".to_string(), 7);
        sample_board.update_value_using_position("b7".to_string(), 4);
        sample_board.update_value_using_position("c2".to_string(), 1);
        sample_board.update_value_using_position("c7".to_string(), 6);
        sample_board.update_value_using_position("c8".to_string(), 5);
        sample_board.update_value_using_position("d1".to_string(), 5);
        sample_board.update_value_using_position("d3".to_string(), 9);
        sample_board.update_value_using_position("d5".to_string(), 3);
        sample_board.update_value_using_position("d7".to_string(), 7);
        sample_board.update_value_using_position("d8".to_string(), 8);
        sample_board.update_value_using_position("e5".to_string(), 7);
        sample_board.update_value_using_position("f2".to_string(), 4);
        sample_board.update_value_using_position("f3".to_string(), 8);
        sample_board.update_value_using_position("f5".to_string(), 2);
        sample_board.update_value_using_position("f7".to_string(), 1);
        sample_board.update_value_using_position("f9".to_string(), 3);
        sample_board.update_value_using_position("g2".to_string(), 5);
        sample_board.update_value_using_position("g3".to_string(), 2);
        sample_board.update_value_using_position("g8".to_string(), 9);
        sample_board.update_value_using_position("h3".to_string(), 1);
        sample_board.update_value_using_position("i1".to_string(), 3);
        sample_board.update_value_using_position("i4".to_string(), 9);
        sample_board.update_value_using_position("i6".to_string(), 2);
        sample_board.update_value_using_position("i9".to_string(), 5);
        return sample_board;
    }

    fn generate_sample_easy() -> Sudoku {
        let mut sample_board = Sudoku::new();
        sample_board.update_value_using_position("a1".to_string(), 4);
        sample_board.update_value_using_position("a3".to_string(), 1);
        sample_board.update_value_using_position("a4".to_string(), 2);
        sample_board.update_value_using_position("a5".to_string(), 9);
        sample_board.update_value_using_position("a8".to_string(), 7);
        sample_board.update_value_using_position("a9".to_string(), 5);
        sample_board.update_value_using_position("b1".to_string(), 2);
        sample_board.update_value_using_position("b4".to_string(), 3);
        sample_board.update_value_using_position("b7".to_string(), 8);
        sample_board.update_value_using_position("c2".to_string(), 7);
        sample_board.update_value_using_position("c5".to_string(), 8);
        sample_board.update_value_using_position("c9".to_string(), 6);
        sample_board.update_value_using_position("d4".to_string(), 1);
        sample_board.update_value_using_position("d6".to_string(), 3);
        sample_board.update_value_using_position("d8".to_string(), 6);
        sample_board.update_value_using_position("d9".to_string(), 2);
        sample_board.update_value_using_position("e1".to_string(), 1);
        sample_board.update_value_using_position("e3".to_string(), 5);
        sample_board.update_value_using_position("e7".to_string(), 4);
        sample_board.update_value_using_position("e9".to_string(), 3);
        sample_board.update_value_using_position("f1".to_string(), 7);
        sample_board.update_value_using_position("f2".to_string(), 3);
        sample_board.update_value_using_position("f4".to_string(), 6);
        sample_board.update_value_using_position("f6".to_string(), 8);
        sample_board.update_value_using_position("g1".to_string(), 6);
        sample_board.update_value_using_position("g5".to_string(), 2);
        sample_board.update_value_using_position("g8".to_string(), 3);
        sample_board.update_value_using_position("h3".to_string(), 7);
        sample_board.update_value_using_position("h6".to_string(), 1);
        sample_board.update_value_using_position("h9".to_string(), 4);
        sample_board.update_value_using_position("i1".to_string(), 8);
        sample_board.update_value_using_position("i2".to_string(), 9);
        sample_board.update_value_using_position("i5".to_string(), 6);
        sample_board.update_value_using_position("i6".to_string(), 5);
        sample_board.update_value_using_position("i7".to_string(), 1);
        sample_board.update_value_using_position("i9".to_string(), 7);
        return sample_board;
    }

    fn generate_target_very_hard() -> Sudoku {
        let mut target_board = Sudoku::new();
        // Row "a"
        target_board.update_value_using_position("a1".to_string(), 6);
        target_board.update_value_using_position("a2".to_string(), 8);
        target_board.update_value_using_position("a3".to_string(), 5);
        target_board.update_value_using_position("a4".to_string(), 4);
        target_board.update_value_using_position("a5".to_string(), 3);
        target_board.update_value_using_position("a6".to_string(), 9);
        target_board.update_value_using_position("a7".to_string(), 2);
        target_board.update_value_using_position("a8".to_string(), 7);
        target_board.update_value_using_position("a9".to_string(), 1);
        // Row "b"
        target_board.update_value_using_position("b1".to_string(), 4);
        target_board.update_value_using_position("b2".to_string(), 9);
        target_board.update_value_using_position("b3".to_string(), 1);
        target_board.update_value_using_position("b4".to_string(), 7);
        target_board.update_value_using_position("b5".to_string(), 8);
        target_board.update_value_using_position("b6".to_string(), 2);
        target_board.update_value_using_position("b7".to_string(), 6);
        target_board.update_value_using_position("b8".to_string(), 5);
        target_board.update_value_using_position("b9".to_string(), 3);
        // Row "c"
        target_board.update_value_using_position("c1".to_string(), 3);
        target_board.update_value_using_position("c2".to_string(), 2);
        target_board.update_value_using_position("c3".to_string(), 7);
        target_board.update_value_using_position("c4".to_string(), 5);
        target_board.update_value_using_position("c5".to_string(), 6);
        target_board.update_value_using_position("c6".to_string(), 1);
        target_board.update_value_using_position("c7".to_string(), 8);
        target_board.update_value_using_position("c8".to_string(), 4);
        target_board.update_value_using_position("c9".to_string(), 9);
        // Row "d"
        target_board.update_value_using_position("d1".to_string(), 9);
        target_board.update_value_using_position("d2".to_string(), 1);
        target_board.update_value_using_position("d3".to_string(), 6);
        target_board.update_value_using_position("d4".to_string(), 3);
        target_board.update_value_using_position("d5".to_string(), 4);
        target_board.update_value_using_position("d6".to_string(), 7);
        target_board.update_value_using_position("d7".to_string(), 5);
        target_board.update_value_using_position("d8".to_string(), 8);
        target_board.update_value_using_position("d9".to_string(), 2);
        // Row "e"
        target_board.update_value_using_position("e1".to_string(), 7);
        target_board.update_value_using_position("e2".to_string(), 5);
        target_board.update_value_using_position("e3".to_string(), 8);
        target_board.update_value_using_position("e4".to_string(), 1);
        target_board.update_value_using_position("e5".to_string(), 2);
        target_board.update_value_using_position("e6".to_string(), 6);
        target_board.update_value_using_position("e7".to_string(), 3);
        target_board.update_value_using_position("e8".to_string(), 9);
        target_board.update_value_using_position("e9".to_string(), 4);
        // Row "f"
        target_board.update_value_using_position("f1".to_string(), 2);
        target_board.update_value_using_position("f2".to_string(), 4);
        target_board.update_value_using_position("f3".to_string(), 3);
        target_board.update_value_using_position("f4".to_string(), 8);
        target_board.update_value_using_position("f5".to_string(), 9);
        target_board.update_value_using_position("f6".to_string(), 5);
        target_board.update_value_using_position("f7".to_string(), 7);
        target_board.update_value_using_position("f8".to_string(), 1);
        target_board.update_value_using_position("f9".to_string(), 6);
        // Row "g"
        target_board.update_value_using_position("g1".to_string(), 1);
        target_board.update_value_using_position("g2".to_string(), 3);
        target_board.update_value_using_position("g3".to_string(), 9);
        target_board.update_value_using_position("g4".to_string(), 6);
        target_board.update_value_using_position("g5".to_string(), 7);
        target_board.update_value_using_position("g6".to_string(), 8);
        target_board.update_value_using_position("g7".to_string(), 4);
        target_board.update_value_using_position("g8".to_string(), 2);
        target_board.update_value_using_position("g9".to_string(), 5);
        // Row "h"
        target_board.update_value_using_position("h1".to_string(), 8);
        target_board.update_value_using_position("h2".to_string(), 6);
        target_board.update_value_using_position("h3".to_string(), 2);
        target_board.update_value_using_position("h4".to_string(), 9);
        target_board.update_value_using_position("h5".to_string(), 5);
        target_board.update_value_using_position("h6".to_string(), 4);
        target_board.update_value_using_position("h7".to_string(), 1);
        target_board.update_value_using_position("h8".to_string(), 3);
        target_board.update_value_using_position("h9".to_string(), 7);
        // Row "i"
        target_board.update_value_using_position("i1".to_string(), 5);
        target_board.update_value_using_position("i2".to_string(), 7);
        target_board.update_value_using_position("i3".to_string(), 4);
        target_board.update_value_using_position("i4".to_string(), 2);
        target_board.update_value_using_position("i5".to_string(), 1);
        target_board.update_value_using_position("i6".to_string(), 3);
        target_board.update_value_using_position("i7".to_string(), 9);
        target_board.update_value_using_position("i8".to_string(), 6);
        target_board.update_value_using_position("i9".to_string(), 8);
        return target_board;
    }

    fn generate_target_hard() -> Sudoku {
        let mut target_board = Sudoku::new();
        // Row "a"
        target_board.update_value_using_position("a1".to_string(), 8);
        target_board.update_value_using_position("a2".to_string(), 3);
        target_board.update_value_using_position("a3".to_string(), 5);
        target_board.update_value_using_position("a4".to_string(), 4);
        target_board.update_value_using_position("a5".to_string(), 1);
        target_board.update_value_using_position("a6".to_string(), 6);
        target_board.update_value_using_position("a7".to_string(), 9);
        target_board.update_value_using_position("a8".to_string(), 2);
        target_board.update_value_using_position("a9".to_string(), 7);
        // Row "b"
        target_board.update_value_using_position("b1".to_string(), 2);
        target_board.update_value_using_position("b2".to_string(), 9);
        target_board.update_value_using_position("b3".to_string(), 6);
        target_board.update_value_using_position("b4".to_string(), 8);
        target_board.update_value_using_position("b5".to_string(), 5);
        target_board.update_value_using_position("b6".to_string(), 7);
        target_board.update_value_using_position("b7".to_string(), 4);
        target_board.update_value_using_position("b8".to_string(), 3);
        target_board.update_value_using_position("b9".to_string(), 1);
        // Row "c"
        target_board.update_value_using_position("c1".to_string(), 4);
        target_board.update_value_using_position("c2".to_string(), 1);
        target_board.update_value_using_position("c3".to_string(), 7);
        target_board.update_value_using_position("c4".to_string(), 2);
        target_board.update_value_using_position("c5".to_string(), 9);
        target_board.update_value_using_position("c6".to_string(), 3);
        target_board.update_value_using_position("c7".to_string(), 6);
        target_board.update_value_using_position("c8".to_string(), 5);
        target_board.update_value_using_position("c9".to_string(), 8);
        // Row "d"
        target_board.update_value_using_position("d1".to_string(), 5);
        target_board.update_value_using_position("d2".to_string(), 6);
        target_board.update_value_using_position("d3".to_string(), 9);
        target_board.update_value_using_position("d4".to_string(), 1);
        target_board.update_value_using_position("d5".to_string(), 3);
        target_board.update_value_using_position("d6".to_string(), 4);
        target_board.update_value_using_position("d7".to_string(), 7);
        target_board.update_value_using_position("d8".to_string(), 8);
        target_board.update_value_using_position("d9".to_string(), 2);
        // Row "e"
        target_board.update_value_using_position("e1".to_string(), 1);
        target_board.update_value_using_position("e2".to_string(), 2);
        target_board.update_value_using_position("e3".to_string(), 3);
        target_board.update_value_using_position("e4".to_string(), 6);
        target_board.update_value_using_position("e5".to_string(), 7);
        target_board.update_value_using_position("e6".to_string(), 8);
        target_board.update_value_using_position("e7".to_string(), 5);
        target_board.update_value_using_position("e8".to_string(), 4);
        target_board.update_value_using_position("e9".to_string(), 9);
        // Row "f"
        target_board.update_value_using_position("f1".to_string(), 7);
        target_board.update_value_using_position("f2".to_string(), 4);
        target_board.update_value_using_position("f3".to_string(), 8);
        target_board.update_value_using_position("f4".to_string(), 5);
        target_board.update_value_using_position("f5".to_string(), 2);
        target_board.update_value_using_position("f6".to_string(), 9);
        target_board.update_value_using_position("f7".to_string(), 1);
        target_board.update_value_using_position("f8".to_string(), 6);
        target_board.update_value_using_position("f9".to_string(), 3);
        // Row "g"
        target_board.update_value_using_position("g1".to_string(), 6);
        target_board.update_value_using_position("g2".to_string(), 5);
        target_board.update_value_using_position("g3".to_string(), 2);
        target_board.update_value_using_position("g4".to_string(), 7);
        target_board.update_value_using_position("g5".to_string(), 8);
        target_board.update_value_using_position("g6".to_string(), 1);
        target_board.update_value_using_position("g7".to_string(), 3);
        target_board.update_value_using_position("g8".to_string(), 9);
        target_board.update_value_using_position("g9".to_string(), 4);
        // Row "h"
        target_board.update_value_using_position("h1".to_string(), 9);
        target_board.update_value_using_position("h2".to_string(), 8);
        target_board.update_value_using_position("h3".to_string(), 1);
        target_board.update_value_using_position("h4".to_string(), 3);
        target_board.update_value_using_position("h5".to_string(), 4);
        target_board.update_value_using_position("h6".to_string(), 5);
        target_board.update_value_using_position("h7".to_string(), 2);
        target_board.update_value_using_position("h8".to_string(), 7);
        target_board.update_value_using_position("h9".to_string(), 6);
        // Row "i"
        target_board.update_value_using_position("i1".to_string(), 3);
        target_board.update_value_using_position("i2".to_string(), 7);
        target_board.update_value_using_position("i3".to_string(), 4);
        target_board.update_value_using_position("i4".to_string(), 9);
        target_board.update_value_using_position("i5".to_string(), 6);
        target_board.update_value_using_position("i6".to_string(), 2);
        target_board.update_value_using_position("i7".to_string(), 8);
        target_board.update_value_using_position("i8".to_string(), 1);
        target_board.update_value_using_position("i9".to_string(), 5);
        return target_board;
    }

    fn generate_target_easy() -> Sudoku {
        let mut target_board = Sudoku::new();
        // Row "a"
        target_board.update_value_using_position("a1".to_string(), 4);
        target_board.update_value_using_position("a2".to_string(), 8);
        target_board.update_value_using_position("a3".to_string(), 1);
        target_board.update_value_using_position("a4".to_string(), 2);
        target_board.update_value_using_position("a5".to_string(), 9);
        target_board.update_value_using_position("a6".to_string(), 6);
        target_board.update_value_using_position("a7".to_string(), 3);
        target_board.update_value_using_position("a8".to_string(), 7);
        target_board.update_value_using_position("a9".to_string(), 5);
        // Row "b"
        target_board.update_value_using_position("b1".to_string(), 2);
        target_board.update_value_using_position("b2".to_string(), 5);
        target_board.update_value_using_position("b3".to_string(), 6);
        target_board.update_value_using_position("b4".to_string(), 3);
        target_board.update_value_using_position("b5".to_string(), 1);
        target_board.update_value_using_position("b6".to_string(), 7);
        target_board.update_value_using_position("b7".to_string(), 8);
        target_board.update_value_using_position("b8".to_string(), 4);
        target_board.update_value_using_position("b9".to_string(), 9);
        // Row "c"
        target_board.update_value_using_position("c1".to_string(), 3);
        target_board.update_value_using_position("c2".to_string(), 7);
        target_board.update_value_using_position("c3".to_string(), 9);
        target_board.update_value_using_position("c4".to_string(), 5);
        target_board.update_value_using_position("c5".to_string(), 8);
        target_board.update_value_using_position("c6".to_string(), 4);
        target_board.update_value_using_position("c7".to_string(), 2);
        target_board.update_value_using_position("c8".to_string(), 1);
        target_board.update_value_using_position("c9".to_string(), 6);
        // Row "d"
        target_board.update_value_using_position("d1".to_string(), 9);
        target_board.update_value_using_position("d2".to_string(), 4);
        target_board.update_value_using_position("d3".to_string(), 8);
        target_board.update_value_using_position("d4".to_string(), 1);
        target_board.update_value_using_position("d5".to_string(), 5);
        target_board.update_value_using_position("d6".to_string(), 3);
        target_board.update_value_using_position("d7".to_string(), 7);
        target_board.update_value_using_position("d8".to_string(), 6);
        target_board.update_value_using_position("d9".to_string(), 2);
        // Row "e"
        target_board.update_value_using_position("e1".to_string(), 1);
        target_board.update_value_using_position("e2".to_string(), 6);
        target_board.update_value_using_position("e3".to_string(), 5);
        target_board.update_value_using_position("e4".to_string(), 9);
        target_board.update_value_using_position("e5".to_string(), 7);
        target_board.update_value_using_position("e6".to_string(), 2);
        target_board.update_value_using_position("e7".to_string(), 4);
        target_board.update_value_using_position("e8".to_string(), 8);
        target_board.update_value_using_position("e9".to_string(), 3);
        // Row "f"
        target_board.update_value_using_position("f1".to_string(), 7);
        target_board.update_value_using_position("f2".to_string(), 3);
        target_board.update_value_using_position("f3".to_string(), 2);
        target_board.update_value_using_position("f4".to_string(), 6);
        target_board.update_value_using_position("f5".to_string(), 4);
        target_board.update_value_using_position("f6".to_string(), 8);
        target_board.update_value_using_position("f7".to_string(), 9);
        target_board.update_value_using_position("f8".to_string(), 5);
        target_board.update_value_using_position("f9".to_string(), 1);
        // Row "g"
        target_board.update_value_using_position("g1".to_string(), 6);
        target_board.update_value_using_position("g2".to_string(), 1);
        target_board.update_value_using_position("g3".to_string(), 4);
        target_board.update_value_using_position("g4".to_string(), 7);
        target_board.update_value_using_position("g5".to_string(), 2);
        target_board.update_value_using_position("g6".to_string(), 9);
        target_board.update_value_using_position("g7".to_string(), 5);
        target_board.update_value_using_position("g8".to_string(), 3);
        target_board.update_value_using_position("g9".to_string(), 8);
        // Row "h"
        target_board.update_value_using_position("h1".to_string(), 5);
        target_board.update_value_using_position("h2".to_string(), 2);
        target_board.update_value_using_position("h3".to_string(), 7);
        target_board.update_value_using_position("h4".to_string(), 8);
        target_board.update_value_using_position("h5".to_string(), 3);
        target_board.update_value_using_position("h6".to_string(), 1);
        target_board.update_value_using_position("h7".to_string(), 6);
        target_board.update_value_using_position("h8".to_string(), 9);
        target_board.update_value_using_position("h9".to_string(), 4);
        // Row "i"
        target_board.update_value_using_position("i1".to_string(), 8);
        target_board.update_value_using_position("i2".to_string(), 9);
        target_board.update_value_using_position("i3".to_string(), 3);
        target_board.update_value_using_position("i4".to_string(), 4);
        target_board.update_value_using_position("i5".to_string(), 6);
        target_board.update_value_using_position("i6".to_string(), 5);
        target_board.update_value_using_position("i7".to_string(), 1);
        target_board.update_value_using_position("i8".to_string(), 2);
        target_board.update_value_using_position("i9".to_string(), 7);
        return target_board;
    }

    #[test]
    fn solve_sudoku_easy() {
        let sample = generate_sample_easy();
        let target = generate_target_easy();
        let mut solver = Solver::new(sample);
        let solved_board = solver.solve_board().unwrap();
        assert_eq!(solved_board, target);
    }

    #[test]
    fn solve_sudoku_hard() {
        let sample = generate_sample_hard();
        let target = generate_target_hard();
        let mut solver = Solver::new(sample);
        let solved_board = solver.solve_board().unwrap();
        assert_eq!(solved_board, target);
    }

    #[test]
    fn solve_sudoku_very_hard() {
        let sample = generate_sample_very_hard();
        let target = generate_target_very_hard();
        let mut solver = Solver::new(sample);
        let solved_board = solver.solve_board().unwrap();
        assert_eq!(solved_board, target);
    }

    #[test]
    fn check_board_rows_ok() {
        let sample = generate_sample_hard();
        let mut solver = Solver::new(sample);
        solver.check_rows().unwrap();
    }

    #[test]
    #[should_panic]
    fn check_board_rows_fail() {
        let mut sample = generate_sample_hard();
        sample.update_value_using_position("a2".to_string(), 8);
        let mut solver = Solver::new(sample);
        solver.check_rows().unwrap();
    }

    #[test]
    fn check_board_columns_ok() {
        let sample = generate_sample_hard();
        let mut solver = Solver::new(sample);
        solver.check_cols().unwrap();
    }

    #[test]
    #[should_panic]
    fn check_board_columns_fail() {
        let mut sample = generate_sample_hard();
        sample.update_value_using_position("b1".to_string(), 8);
        let mut solver = Solver::new(sample);
        solver.check_cols().unwrap();
    }
    #[test]
    #[should_panic]
    fn check_board_subsquares_fail() {
        let mut sample = generate_sample_hard();
        sample.update_value_using_position("h7".to_string(), 5);
        let mut solver = Solver::new(sample);
        solver.check_subsquares().unwrap();
    }
}
