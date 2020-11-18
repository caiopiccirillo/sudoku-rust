mod reader;
mod solver;
mod writer;

fn main() {
    println!("Begin!");
    let mut my_reader: reader::Reader;
    my_reader.read_image("/home/caio/Documentos/sudoku-rust/assets/sample.png".to_string());
        
}
