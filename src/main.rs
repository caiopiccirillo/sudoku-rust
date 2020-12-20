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
    let mut sudoku_board = sudoku::Sudoku::new();
    let my_solver = solver::Solver::new(sudoku_board);
    //println!("{:?}", sudoku_board);
    //my_reader.process_image(&sudoku_board);
    println!("{:?}", my_reader.img.dimensions());
}
