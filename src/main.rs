use image::GenericImageView;
use std::env;

mod reader;
mod solver;
mod sudoku;
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
    println!("{:?}", sudoku_board);
    my_reader.process_image(&sudoku_board);
    println!("{:?}", my_reader.img.dimensions());
}
