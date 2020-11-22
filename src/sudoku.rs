use std::{collections::HashMap, fmt::Debug};

pub struct Sudoku {
    pub board: HashMap<String, Option<u8>>,
    rows: Vec<char>,
    cols: Vec<char>,
}

impl Sudoku {
    pub fn new() -> Self {
        let rows = (b'a'..=b'i') // Start as u8
            .map(|c| c as char) // Convert all to chars
            .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
            .collect::<Vec<_>>(); // Collect as Vec<char>
        let cols = (b'1'..=b'9') // Start as u8
            .map(|c| c as char) // Convert all to chars
            .filter(|c| c.is_numeric()) // Filter only numeric chars
            .collect::<Vec<_>>(); // Collect as Vec<char>
        let mut board: HashMap<String, Option<u8>> = HashMap::new();
        for r in rows.iter() {
            for c in cols.iter() {
                // Add None value to create an empty board
                board.insert(format!("{}{}", r, c), None);
            }
        }
        Self { board, rows, cols }
    }
    pub fn update_value_using_position(&mut self, position: String, value: u8) {
        self.board.insert(position, Some(value));
    }
    pub fn update_value_using_row_col(&mut self, row: char, col: char, value: u8) {
        self.update_value_using_position(format!("{}{}", row, col), value);
    }
}

impl Debug for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result: std::fmt::Result = Ok(());
        writeln!(f, "  ")?;
        write!(f, "  ")?;
        // Write each column name
        for c in self.cols.iter() {
            write!(f, "|{}", c)?;
        }
        writeln!(f, "|  ")?;
        // Write the value of all elements in board
        for r in self.rows.iter() {
            // Write each row name
            write!(f, "|{}", r)?;
            for c in self.cols.iter() {
                // Get value stored in board
                match self.board.get(&format!("{}{}", r, c)).unwrap() {
                    Some(value) => write!(f, "|{}", value)?,
                    None => write!(f, "|-")?,
                };
            }
            writeln!(f, "|  ")?;
        }
        return result;
    }
}
