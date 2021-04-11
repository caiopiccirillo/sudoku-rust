use std::env;

mod reader;
mod solver;
mod sudoku;
mod writer;

fn main() {
    // Read arguments
    let args: Vec<String> = env::args().collect();
    // Get image file path
    let path = &args[1];
    // Create an instance of reader
    let mut my_reader = reader::Reader::new(path.into());
    let sudoku_board = my_reader.process_image();
    println!("Sudoku board: {:?}", sudoku_board);
    let mut my_solver = solver::Solver::new(sudoku_board);
    let result = my_solver.solve_board();
    match result {
        Ok(solved_board) => {
            println!("Solved board: {:?}", solved_board);
            let _my_writer = writer::Writer::new();
        }
        Err(err) => {
            println!("Error while solving board: {:?}", err);
        }
    }
}
