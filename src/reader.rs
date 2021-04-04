use super::sudoku;
/// This module will read the image file and extract the information from its file.
// use imageproc::contours::find_contours_with_threshold;
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
    pub fn _read_image(&mut self, path: String) {
        self.img = image::open(path).unwrap()
    }
    pub fn _process_image(&self, _board: &sudoku::Sudoku) {
        //let _image = find_contours_with_threshold::<u8>(&self.img.into_luma8(), 125);
        //println!("{:?}",board);
    }
}
