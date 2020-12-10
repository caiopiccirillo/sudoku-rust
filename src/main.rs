use std::env;
use image::GenericImageView;

mod reader;
mod solver;
mod writer;
mod sudoku;
mod tests;

fn main() {
    // Read arguments
    let args: Vec<String> = env::args().collect();
    // Get image file path
    let path = &args[1];
    // Create an instance of reader
    let my_reader= reader::Reader::new(path.into());
    let mut sudoku_board = sudoku::Sudoku::new();
    println!("{:?}",sudoku_board);
    my_reader.process_image(&sudoku_board);
    println!("{:?}",my_reader.img.dimensions());
}