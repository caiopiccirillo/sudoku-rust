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
        let result: std::fmt::Result = Ok(());
        write!(f, "  ")?;
        for c in self.cols.iter() {
            write!(f, "|{}", c)?;
        }
        writeln!(f, "|  ")?;
        for r in self.rows.iter() {
            write!(f, "|{}",r)?;
            for c in self.cols.iter() {
                // Add None value to create an empty board
                match self.board.get(&format!("{}{}", r, c)).unwrap() {
                    Some(value) => write!(f, "|{}", value)?,
                    None => write!(f, "|-")?
                };
            }
            writeln!(f, "|  ")?;
        }
        return result;
    }
}
