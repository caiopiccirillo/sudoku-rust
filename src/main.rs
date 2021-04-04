use image::GenericImageView;
use std::env;

mod reader;
mod solver;
mod sudoku;
#[cfg(test)]
mod tests;
mod writer;

fn main() {
    // Read arguments
    let args: Vec<String> = env::args().collect();
    // Get image file path
    let path = &args[1];
    // Create an instance of reader
    let my_reader = reader::Reader::new(path.into());
    let sudoku_board = sudoku::Sudoku::new();
    println!("{:?}", sudoku_board);
    let mut my_solver = solver::Solver::new(sudoku_board);
    let solved_board = my_solver.solve_board();
    println!("{:?}", solved_board);
    let _my_writer = writer::Writer::new();
    //my_reader.process_image(&sudoku_board);
    println!("{:?}", my_reader.img.dimensions());
}
