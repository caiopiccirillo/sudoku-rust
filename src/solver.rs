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
            .map(Some) // Convert all to Option
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

    fn check_group(&mut self, indexes: &mut Vec<String>, group_name: &str) -> Result<(), ()> {
        let mut values = Vec::<Option<u8>>::new();
        for index_key in indexes.iter() {
            values.push(self.data.board[index_key]);
        }

        for i in 0..values.len() {
            for j in 0..values.len() {
                match values[i] {
                    Some(v1) => {
                        // If the cell has a value, ensure its `possible_numbers` list contains only that value.
                        if let Some(pos) = self.possible_numbers[&indexes[i]]
                            .iter()
                            .position(|x| *x != Some(v1))
                        {
                            self.possible_numbers
                                .get_mut(&indexes[i])
                                .unwrap()
                                .remove(pos);
                        }
                        match values[j] {
                            Some(v2) => {
                                // If two cells in the group have the same value, it's an error.
                                if v1 == v2 && i != j {
                                    println!(
                                        "Error in {}: value {} at index {} conflicts with value {} at index {}",
                                        group_name, v1, indexes[i], v2, indexes[j]
                                    );
                                    return Err(());
                                }
                            }
                            // If cell j is empty, remove v1 from its `possible_numbers`.
                            None => {
                                if let Some(pos) = self.possible_numbers[&indexes[j]]
                                    .iter()
                                    .position(|x| *x == Some(v1))
                                {
                                    self.possible_numbers
                                        .get_mut(&indexes[j])
                                        .unwrap()
                                        .remove(pos);
                                }
                            }
                        }
                    }
                    None => {
                        // If cell i is empty, continue to the next cell.
                        continue;
                    }
                }
            }
        }
        self.unique_possibility(indexes)?;
        self.naked_twin_strategy(indexes)?;
        Ok(())
    }

    fn check_rows(&mut self) -> Result<(), ()> {
        for row_char in self.data.rows.clone() {
            let mut indexes: Vec<String> = self
                .data
                .cols
                .clone()
                .iter()
                .map(|col_char| format!("{}{}", row_char, col_char))
                .collect();
            self.check_group(&mut indexes, &format!("row {}", row_char))?;
        }
        Ok(())
    }

    fn check_cols(&mut self) -> Result<(), ()> {
        for col_char in self.data.cols.clone() {
            let mut indexes: Vec<String> = self
                .data
                .rows
                .clone()
                .iter()
                .map(|row_char| format!("{}{}", row_char, col_char))
                .collect();
            self.check_group(&mut indexes, &format!("column {}", col_char))?;
        }
        Ok(())
    }

    fn check_subsquares(&mut self) -> Result<(), ()> {
        let mut subsquare_id = 0;
        for row_offset_group in (0..9).step_by(3) {
            for col_offset_group in (0..9).step_by(3) {
                let mut indexes: Vec<String> = Vec::new();
                for i in 0..3 {
                    for j in 0..3 {
                        indexes.push(format!(
                            "{}{}",
                            self.data.rows[i + row_offset_group],
                            self.data.cols[j + col_offset_group]
                        ));
                    }
                }
                self.check_group(&mut indexes, &format!("subsquare {}", subsquare_id))?;
                subsquare_id += 1;
            }
        }
        Ok(())
    }

    // unique_possibility checks if a number is the only possible option for a cell within a group (row, column, or subsquare).
    // If so, it means that number must be in that cell.
    // This function then removes all other possibilities from that cell.
    // Example: If in a row, only cell 'a1' can contain '5', then '5' is placed in 'a1'.
    fn unique_possibility(&mut self, list_index_by_group: &mut [String]) -> Result<(), ()> {
        if list_index_by_group.is_empty() {
            return Err(());
        }
        for index1 in list_index_by_group.iter().cloned() {
            if self.possible_numbers[&index1].len() >= 2 {
                for value1 in self.possible_numbers[&index1].clone() {
                    let mut unique = true;
                    for index2 in list_index_by_group.iter().cloned() {
                        if index1 != index2 {
                            for value2_opt in self.possible_numbers[&index2].clone() {
                                // Both value1 (outer loop variable, which is an Option<u8>) 
                                // and value2_opt (inner loop variable) are Option<u8>.
                                // We only care if they are both Some and equal.
                                if value1.is_some() && value1 == value2_opt {
                                    unique = false;
                                    break; 
                                }
                            }
                        }
                        if !unique { break; } // break from inner loop over index2
                    }
                    if unique {
                        // value1 is an Option<u8>. If it's Some(v), then v is unique for cell index1 in this group.
                        if let Some(unique_val) = value1 {
                            // println!(
                            //     "Found unique: {} for cell {}",
                            // unique_val, index1
                            // );
                            // This cell (index1) must contain unique_val.
                            // So, remove all other possibilities from this cell's list.
                            if let Some(poss_list) = self.possible_numbers.get_mut(&index1) {
                                poss_list.retain(|p| *p == Some(unique_val));
                            } else {
                                // This would be an internal error, index1 should always be a valid key
                                eprintln!("Error: Key {} not found in possible_numbers during unique_possibility.", index1);
                                return Err(()); // Propagate error
                            }
                        }
                        // If value1 is None, it cannot be a unique possibility to set a cell.
                        // So, we do nothing if value1 is None.
                    }
                }
            }
        }
        Ok(())
    }

    // naked_twin_strategy identifies "naked twins" within a group.
    // Naked twins are two cells in the same group (row, col, or subsquare) that both have the exact same two candidate numbers.
    // For example, if cell 'a1' can only be '2' or '3', and cell 'a5' can also only be '2' or '3',
    // then '2' and '3' must be in 'a1' and 'a5' (in some order).
    // Therefore, '2' and '3' can be removed as candidates from all other cells in that same group.
    fn naked_twin_strategy(&mut self, list_index_by_group: &mut [String]) -> Result<(), ()> {
        if list_index_by_group.is_empty() {
            return Err(());
        }
        let mut first_twin_key: Option<String> = None;
        let mut second_twin_key: Option<String> = None;

        // Find a pair of naked twins
        'outer_loop: for i in 0..list_index_by_group.len() {
            let key1 = &list_index_by_group[i];
            // Ensure key1 exists, though it should if list_index_by_group is built correctly
            let poss1 = self.possible_numbers.get(key1).ok_or_else(|| {
                eprintln!("Error: Key {} not found in possible_numbers (naked_twin_strategy).", key1);
                () 
            })?;
            
            // A cell qualifies for naked twin if it has exactly two *actual* number possibilities
            let actual_poss1: Vec<u8> = poss1.iter().filter_map(|&p| p).collect();
            if actual_poss1.len() == 2 {
                for j in (i + 1)..list_index_by_group.len() {
                    let key2 = &list_index_by_group[j];
                    let poss2 = self.possible_numbers.get(key2).ok_or_else(|| {
                        eprintln!("Error: Key {} not found in possible_numbers (naked_twin_strategy).", key2);
                        ()
                    })?;

                    let actual_poss2: Vec<u8> = poss2.iter().filter_map(|&p| p).collect();
                    if actual_poss2.len() == 2 && actual_poss1 == actual_poss2 { // Compare actual numbers
                        first_twin_key = Some(key1.clone());
                        second_twin_key = Some(key2.clone());
                        break 'outer_loop; // Found a pair, exit both loops
                    }
                }
            }
        }

        if let (Some(key1), Some(key2)) = (first_twin_key, second_twin_key) {
            // These are Vec<Option<u8>>, but we need the Some(u8) values for comparison.
            // We already know they both have two Some(u8) values and these are the same.
            let twin_actual_values: Vec<u8> = self.possible_numbers[&key1]
                .iter()
                .filter_map(|&p| p)
                .collect();

            // Should be exactly 2 values, but good to be safe
            if twin_actual_values.len() == 2 {
                let val1_to_remove = Some(twin_actual_values[0]);
                let val2_to_remove = Some(twin_actual_values[1]);

                for cell_key_str in list_index_by_group.iter() {
                    if *cell_key_str != key1 && *cell_key_str != key2 {
                        let possibilities = self.possible_numbers.get_mut(cell_key_str).ok_or_else(|| {
                            eprintln!("Error: Key {} not found in possible_numbers (naked_twin_strategy removal).", cell_key_str);
                            ()
                        })?;
                        
                        let original_len = possibilities.iter().filter(|p| p.is_some()).count();
                        possibilities.retain(|p| *p != val1_to_remove && *p != val2_to_remove);
                        let new_len = possibilities.iter().filter(|p| p.is_some()).count();

                        if new_len == 0 && original_len > 0 && self.data.board[cell_key_str].is_none() {
                            // A non-filled cell lost all its possibilities due to this rule. This is a contradiction.
                            println!("Error: Naked twin strategy led to cell {} (not part of twin) having no possibilities.", cell_key_str);
                            return Err(()); 
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn apply_solved_cells(&mut self) {
        for row in self.data.rows.clone() {
            for col in self.data.cols.clone() {
                let cell_key = format!("{}{}", row, col);
                // Check if the key exists before trying to access its possibilities
                if let Some(possibilities) = self.possible_numbers.get(&cell_key) {
                    if possibilities.len() == 1 {
                        if let Some(value_to_set) = possibilities[0] {
                            self.data.update_value_using_position(
                                cell_key.clone(), // update_value_using_position takes String
                                value_to_set,
                            );
                        } else {
                            // This state (len == 1 but element is None) implies a logic error.
                            // A cell with one "possibility" should not have that possibility be None.
                            eprintln!("Error: Cell {} has a single possibility which is None.", cell_key);
                            // Depending on desired robustness, this could be escalated to an error
                            // that stops the solver, as it indicates a corrupted state.
                        }
                    }
                } else {
                     eprintln!("Error: Key {} not found in possible_numbers during apply_solved_cells.", cell_key);
                    // This case implies a critical logic error as possible_numbers should be initialized
                    // for all cells. This could panic or return an error to halt execution.
                    // For now, just logging, but this is severe.
                }
            }
        }
    }

    fn find_cell_to_guess(&self) -> Option<String> {
        let mut best_cell_key: Option<String> = None;
        let mut min_actual_possibilities = 10; // Represents count of Some(value), not vec.len()

        for r_char in self.data.rows.iter() {
            for c_char in self.data.cols.iter() {
                let cell_key = format!("{}{}", r_char, c_char);
                
                // Ensure the key exists in both board and possible_numbers before using it.
                if self.data.board.get(&cell_key).is_some() && 
                   self.possible_numbers.get(&cell_key).is_some() {
                    
                    if self.data.board[&cell_key].is_none() { // If cell is empty
                        // Count only Some(value) possibilities
                        let actual_num_possibilities = self.possible_numbers[&cell_key]
                            .iter()
                            .filter(|p| p.is_some())
                            .count();

                        if actual_num_possibilities > 1 && actual_num_possibilities < min_actual_possibilities {
                            min_actual_possibilities = actual_num_possibilities;
                            best_cell_key = Some(cell_key.clone());
                            // Optimization: if a cell with 2 actual possibilities is found, it's a good candidate.
                            if min_actual_possibilities == 2 { 
                                return best_cell_key;
                            }
                        }
                    }
                } else {
                    eprintln!("Error: Key {} missing in board or possible_numbers during find_cell_to_guess.", cell_key);
                    // This indicates a critical logic error. The program state might be corrupted.
                    // Depending on overall error strategy, could panic here or return a special error.
                    // For now, it just logs and might lead to returning None or a suboptimal guess.
                }
            }
        }
        best_cell_key
    }

    fn recursive_solve(&mut self) -> Result<Sudoku, &str> {
        if let Some(cell_to_guess) = self.find_cell_to_guess() {
            // Ensure cell_to_guess exists in possible_numbers before proceeding
            let original_possibilities = match self.possible_numbers.get(&cell_to_guess) {
                Some(poss) => poss.clone(), // Clone to avoid borrowing issues while modifying map later
                None => {
                    eprintln!("Error: cell_to_guess {} not found in possible_numbers.", cell_to_guess);
                    return Err("Internal error: Guessed cell not in possibility map");
                }
            };
            
            // Iterate only over actual number possibilities (filter out None)
            let valid_possibilities_to_try: Vec<u8> = original_possibilities
                .iter()
                .filter_map(|&p| p) // Dereference and filter_map Option<u8> to u8
                .collect();

            if valid_possibilities_to_try.is_empty() {
                 // This check is important. If find_cell_to_guess returned a cell,
                 // it should have >1 actual possibilities. If now it's empty, something is wrong.
                 // Or, if the cell is already filled (which find_cell_to_guess should prevent).
                 if self.data.board.get(&cell_to_guess).map_or(true, |v_opt| v_opt.is_none()) {
                    eprintln!("Error: Cell {} (picked for guessing) has no valid number possibilities to try.", cell_to_guess);
                    return Err("Contradiction: Cell chosen for guess has no valid possibilities.");
                 }
                 // If cell is already filled, find_cell_to_guess made a mistake.
            }

            for guess_value in valid_possibilities_to_try {
                let data_clone = self.data.clone();
                let possible_numbers_clone = self.possible_numbers.clone();

                self.data.update_value_using_position(cell_to_guess.clone(), guess_value);
                
                // Update the possibilities for the guessed cell to *only* be the guessed value.
                if let Some(poss_list) = self.possible_numbers.get_mut(&cell_to_guess) {
                    poss_list.clear();
                    poss_list.push(Some(guess_value));
                } else {
                    // This should ideally not happen if cell_to_guess was valid moments ago.
                    eprintln!("Error: cell_to_guess {} disappeared from possible_numbers during guess application.", cell_to_guess);
                    // Restore state before returning error, as we made a change to self.data
                    self.data = data_clone;
                    // self.possible_numbers is already fine as the problematic get_mut failed.
                    return Err("Internal error: Guessed cell vanished from possibility map while updating");
                }
                
                // Recursively call the main solver.
                match self.solve_board() { 
                    Ok(board) => return Ok(board), // Solution found!
                    Err(_) => {
                        // Backtrack: Restore previous state before trying the next guess.
                        self.data = data_clone;
                        self.possible_numbers = possible_numbers_clone;
                        // Continue to the next `guess_value` in the loop.
                    }
                }
            }
            // If all guesses for `cell_to_guess` failed.
            Err("All guesses failed for the chosen cell.")
        } else {
            // No cell was found to make a guess on.
            // This means either the board is solved, or it's in an unsolvable state
            // where no empty cells have valid possibilities to try.
            if self.data.num_empty_fields() == 0 {
                // Board is full. A final consistency check is good.
                match self.check_board_consistency() {
                    Ok(_) => Ok(self.data.clone()),
                    Err(_) => Err("Solved board is inconsistent."), // Should ideally be caught earlier
                }
            } else {
                // Board is not full, but no cell could be picked for guessing.
                // This implies a contradiction (e.g., an empty cell has zero possibilities).
                Err("Board unsolvable: No valid guess options available, but board is not full.")
            }
        }
    }

    pub fn solve_board(&mut self) -> Result<Sudoku, &str> {
        if self.instant.elapsed().as_millis() >= TIMEOUT {
            return Err("Timeout error before solving loop");
        }
        
        let mut previous_num_empty_fields = self.data.num_empty_fields();
        
        if previous_num_empty_fields == 81 && self.data.board.values().all(|v| v.is_none()) {
             // Standard empty board check, but ensure it's truly all None.
            return Err("Empty sudoku board");
        }

        // Initial consistency check for pre-filled boards to catch errors early.
        if previous_num_empty_fields < 81 { // Only if not an empty board
            if self.check_board_consistency().is_err() {
                return Err("Initial board is inconsistent");
            }
        }
        
        // let mut epoch = 1; // Epoch tracking removed as it was unused
        loop {
            if self.instant.elapsed().as_millis() >= TIMEOUT {
                return Err("Timeout error during solving loop");
            }

            // Perform logical checks and update possibilities.
            if self.check_board().is_err() {
                 // check_board itself (via check_group) returns Err if an inconsistency is found.
                 return Err("Checking board error found inconsistency");
            }
            
            self.apply_solved_cells(); // Apply cells that now have only one possibility.

            let current_empty_fields = self.data.num_empty_fields();

            if current_empty_fields == 0 {
                // Board is full. Verify final consistency.
                return match self.check_board_consistency() {
                    Ok(_) => Ok(self.data.clone()),
                    Err(_) => Err("Solved board is inconsistent"), // Should be rare if check_board is thorough
                };
            }

            if previous_num_empty_fields == current_empty_fields {
                // No progress made by deduction, must try guessing.
                return self.recursive_solve();
            }
            
            // Progress was made, continue the loop.
            previous_num_empty_fields = current_empty_fields;
            // println!("Epoch {}", epoch); // Useful for debugging, but remove for production.
            // epoch += 1; // Comment out epoch increment as it's unused
        }
        // The loop should ideally only be exited by a return statement from within.
        // If execution reaches here, it implies an unexpected state or missed condition.
        // However, given the logic, if num_empty_fields > 0 and no progress, recursive_solve is called.
        // If num_empty_fields == 0, it returns Ok. Timeouts also return.
        // This part should ideally be unreachable if logic is sound.
        // For safety, ensure a final check or error.
        // Err("Solver loop terminated unexpectedly without resolution.")
    }

    // Helper function to check overall board consistency (no duplicates in rows, cols, subsquares)
    // This is more comprehensive than check_group as it looks at the whole board state.
    fn check_board_consistency(&self) -> Result<(), ()> {
        // Check rows
        for r_char in self.data.rows.iter() {
            let mut seen_numbers = std::collections::HashSet::new();
            for c_char in self.data.cols.iter() {
                // Use get() for safety, then check Option<Option<u8>>
                if let Some(Some(val)) = self.data.board.get(&format!("{}{}", r_char, c_char)) {
                    if !seen_numbers.insert(*val) { 
                        // eprintln!("Consistency error in row {}: duplicate {}", r_char, val);
                        return Err(()); 
                    }
                }
            }
        }
        // Check columns
        for c_char in self.data.cols.iter() {
            let mut seen_numbers = std::collections::HashSet::new();
            for r_char in self.data.rows.iter() {
                if let Some(Some(val)) = self.data.board.get(&format!("{}{}", r_char, c_char)) {
                    if !seen_numbers.insert(*val) { 
                        // eprintln!("Consistency error in col {}: duplicate {}", c_char, val);
                        return Err(()); 
                    }
                }
            }
        }
        // Check subsquares
        for row_offset_group in (0..9).step_by(3) {
            for col_offset_group in (0..9).step_by(3) {
                let mut seen_numbers = std::collections::HashSet::new();
                for i in 0..3 {
                    for j in 0..3 {
                        let key = format!(
                            "{}{}",
                            self.data.rows[i + row_offset_group],
                            self.data.cols[j + col_offset_group]
                        );
                        if let Some(Some(val)) = self.data.board.get(&key) {
                            if !seen_numbers.insert(*val) { 
                                // eprintln!("Consistency error in subsquare starting at {}{}: duplicate {}", 
                                // self.data.rows[row_offset_group], self.data.cols[col_offset_group], val);
                                return Err(()); 
                            }
                        }
                    }
                }
            }
        }
        Ok(())
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
        sample_board
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
        sample_board
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
        sample_board
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
        target_board
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
        target_board
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
        target_board
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

    #[test]
    fn solve_empty_board() {
        let empty_board = Sudoku::new(); // Assuming Sudoku::new() creates a completely empty board
        let mut solver = Solver::new(empty_board);
        match solver.solve_board() {
            Err(msg) => assert_eq!(msg, "Empty sudoku board"),
            Ok(_) => panic!("Solver should have returned Err for empty board"),
        }
    }

    #[test]
    fn solve_initially_inconsistent_board_row() {
        let mut board = Sudoku::new();
        board.update_value_using_position("a1".to_string(), 5);
        board.update_value_using_position("a2".to_string(), 5); // Duplicate in row 'a'
        let mut solver = Solver::new(board);
        match solver.solve_board() {
            Err(msg) => assert_eq!(msg, "Initial board is inconsistent"),
            Ok(_) => panic!("Solver should have returned Err for inconsistent board"),
        }
    }

    #[test]
    fn solve_initially_inconsistent_board_col() {
        let mut board = Sudoku::new();
        board.update_value_using_position("a1".to_string(), 5);
        board.update_value_using_position("b1".to_string(), 5); // Duplicate in col '1'
        let mut solver = Solver::new(board);
        match solver.solve_board() {
            Err(msg) => assert_eq!(msg, "Initial board is inconsistent"),
            Ok(_) => panic!("Solver should have returned Err for inconsistent board"),
        }
    }

    #[test]
    fn solve_initially_inconsistent_board_subsquare() {
        let mut board = Sudoku::new();
        board.update_value_using_position("a1".to_string(), 5);
        board.update_value_using_position("b2".to_string(), 5); // Duplicate in first subsquare
        let mut solver = Solver::new(board);
        match solver.solve_board() {
            Err(msg) => assert_eq!(msg, "Initial board is inconsistent"),
            Ok(_) => panic!("Solver should have returned Err for inconsistent board"),
        }
    }

    #[test]
    fn solve_unsolvable_board() {
        let mut board = Sudoku::new();
        // Example from https://stackoverflow.com/questions/10973343/sudoku-puzzle-with-no-solution
        // This one is known to be valid but have no solution.
        board.update_value_using_position("a5".to_string(), 1);
        board.update_value_using_position("a7".to_string(), 2);
        board.update_value_using_position("b4".to_string(), 3);
        board.update_value_using_position("b5".to_string(), 4);
        board.update_value_using_position("b8".to_string(), 5);
        board.update_value_using_position("c1".to_string(), 6);
        board.update_value_using_position("c6".to_string(), 7);
        board.update_value_using_position("c9".to_string(), 8);
        board.update_value_using_position("d2".to_string(), 7);
        board.update_value_using_position("d7".to_string(), 1);
        board.update_value_using_position("e1".to_string(), 2);
        board.update_value_using_position("e4".to_string(), 8);
        board.update_value_using_position("e6".to_string(), 3);
        board.update_value_using_position("e9".to_string(), 4);
        board.update_value_using_position("f3".to_string(), 5);
        board.update_value_using_position("f8".to_string(), 6);
        board.update_value_using_position("g1".to_string(), 1);
        board.update_value_using_position("g4".to_string(), 6);
        board.update_value_using_position("g9".to_string(), 7);
        board.update_value_using_position("h2".to_string(), 8);
        board.update_value_using_position("h5".to_string(), 2);
        board.update_value_using_position("h6".to_string(), 1);
        board.update_value_using_position("i3".to_string(), 7);
        board.update_value_using_position("i5".to_string(), 5);
        
        let mut solver = Solver::new(board);
        let result = solver.solve_board();
        assert!(result.is_err(), "Expected an error for unsolvable board, but got Ok: {:?}", result.ok());
        // Check for one of the expected error messages for unsolvable boards
        if result.is_err() { // Make sure err exists before unwrap
            let err_msg = result.err().unwrap();
            assert!(
                err_msg == "All guesses failed for the chosen cell." || 
                err_msg == "Board unsolvable: No valid guess options available, but board is not full." ||
                err_msg == "Contradiction: Cell chosen for guess has no valid possibilities." ||
                err_msg == "Checking board error found inconsistency" || // Added this
                err_msg == "Initial board is inconsistent", // Added this in case check_board_consistency catches it first
                "Unexpected error message for unsolvable board: {}", err_msg
            );
        }
    }
}
