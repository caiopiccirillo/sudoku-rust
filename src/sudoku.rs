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
}

impl Debug for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result: std::fmt::Result = Ok(());
        match write!(f, "  ") {
            Ok(v) => result = Ok(v),
            Err(e) => return Err(e),
        }
        for c in self.cols.iter() {
            match write!(f, "|{}", c) {
                Ok(v) => result = Ok(v),
                Err(e) => return Err(e),
            }
        }
        match writeln!(f, "|  ") {
            Ok(v) => result = Ok(v),
            Err(e) => return Err(e),
        }
        for r in self.rows.iter() {
            match write!(f, "|{}",r) {
                Ok(v) => result = Ok(v),
                Err(e) => return Err(e),
            }
            for c in self.cols.iter() {
                // Add None value to create an empty board
                match self.board.get(&format!("{}{}", r, c)).unwrap() {
                    Some(value) => match write!(f, "|{}", value) {
                        Ok(v) => result = Ok(v),
                        Err(e) => return Err(e),
                    },
                    None => match write!(f, "|-") {
                        Ok(v) => result = Ok(v),
                        Err(e) => return Err(e),
                    },
                };
            }
            match writeln!(f, "|  ") {
                Ok(v) => result = Ok(v),
                Err(e) => return Err(e),
            }
        }
        return result;
    }
}
