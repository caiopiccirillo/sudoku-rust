/// This module will read the image file and extract the information from its file.
use super::sudoku;
pub struct Reader {
    // TODO: img to Some(Box<DynamicImage>)
    pub img: image::DynamicImage,
}

impl Reader {
    pub fn new(path: String) -> Self {
        Self {
            img: image::open(path).unwrap(),
        }
    }
    pub fn read_image(&mut self, path: String) {
        self.img = image::open(path).unwrap()
    }
    pub fn process_image(&self, board: sudoku::Sudoku) {
        todo!();
        self.img.save("assets/test.png").unwrap();
    }
}

impl std::fmt::Debug for Reader{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       todo!()
    }
}
