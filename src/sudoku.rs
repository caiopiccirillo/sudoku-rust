use std::{fmt::Debug, collections::HashMap};

pub struct Sudoku {
    pub board: HashMap<String, Option<u8>>,
}

impl Sudoku {
    pub fn new() -> Self {
        let rows = (b'a'..=b'i') // Start as u8
            .map(|c| c as char) // Convert all to chars
            .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
            .collect::<Vec<_>>(); // Collect as Vec<char>
        let cols = (b'1'..=b'9') // Start as u8
            .map(|c| c as char) // Convert all to chars
            .filter(|c| c.is_numeric()) // Filter only alphabetic chars
            .collect::<Vec<_>>(); // Collect as Vec<char>
        let mut board:HashMap<String, Option<u8>> = HashMap::new();
        for r in rows.iter(){
            for c in cols.iter(){
                board.insert(format!("{}{}",r,c), None);
            }
        }
        Self { board }
    }
}

impl Debug for Sudoku{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
