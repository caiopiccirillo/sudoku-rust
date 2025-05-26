use super::sudoku::*;
use image::*;
use imageproc::contours::*;
/// This module will read the image file and extract the information from its file.
// use imageproc::contours::find_contours_with_threshold;
pub struct Reader {
    // TODO: img to Some(Box<DynamicImage>)
    img: image::DynamicImage,
    board: Sudoku,
}

// Define a custom error type for the reader module
#[derive(Debug)]
pub enum ReaderError {
    ImageOpenError(image::ImageError),
    // Add other potential error types here if needed
}

impl std::fmt::Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReaderError::ImageOpenError(e) => write!(f, "Failed to open image: {}", e),
        }
    }
}

impl std::error::Error for ReaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReaderError::ImageOpenError(e) => Some(e),
        }
    }
}

impl Reader {
    pub fn new(path: String) -> Result<Self, ReaderError> {
        match image::open(&path) {
            Ok(img_dyn) => Ok(Self {
                img: img_dyn,
                board: Sudoku::new(),
            }),
            Err(e) => {
                eprintln!("Error opening image file at path '{}': {}", path, e);
                Err(ReaderError::ImageOpenError(e))
            }
        }
    }
    pub fn process_image(&mut self) -> Sudoku {
        println!("Image dimensions: {:?}", self.img.dimensions());
        let gray_img = self.img.grayscale();
        let contours = find_contours_with_threshold::<i32>(&gray_img.into_luma8(),127);
        println!("Number of contours: {}", contours.len());
        self.board.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sample_very_hard() -> Sudoku {
        let mut sample_board = Sudoku::new();
        sample_board.update_value_using_position("a6".to_string(), 9);
        sample_board.update_value_using_position("a8".to_string(), 7);
        sample_board.update_value_using_position("b5".to_string(), 8);
        sample_board.update_value_using_position("b6".to_string(), 2);
        sample_board.update_value_using_position("b8".to_string(), 5);
        sample_board.update_value_using_position("c1".to_string(), 3);
        sample_board.update_value_using_position("c2".to_string(), 2);
        sample_board.update_value_using_position("c3".to_string(), 7);
        sample_board.update_value_using_position("c8".to_string(), 4);
        sample_board.update_value_using_position("d2".to_string(), 1);
        sample_board.update_value_using_position("d3".to_string(), 6);
        sample_board.update_value_using_position("d5".to_string(), 4);
        sample_board.update_value_using_position("e2".to_string(), 5);
        sample_board.update_value_using_position("e7".to_string(), 3);
        sample_board.update_value_using_position("f5".to_string(), 9);
        sample_board.update_value_using_position("f7".to_string(), 7);
        sample_board.update_value_using_position("g4".to_string(), 6);
        sample_board.update_value_using_position("g9".to_string(), 5);
        sample_board.update_value_using_position("h1".to_string(), 8);
        sample_board.update_value_using_position("h3".to_string(), 2);
        sample_board.update_value_using_position("i3".to_string(), 4);
        sample_board.update_value_using_position("i4".to_string(), 2);
        sample_board.update_value_using_position("i9".to_string(), 8);
        sample_board
    }

    fn generate_sample_hard() -> Sudoku {
        let mut sample_board = Sudoku::new();
        sample_board.update_value_using_position("a1".to_string(), 8);
        sample_board.update_value_using_position("a4".to_string(), 4);
        sample_board.update_value_using_position("a6".to_string(), 6);
        sample_board.update_value_using_position("a9".to_string(), 7);
        sample_board.update_value_using_position("b7".to_string(), 4);
        sample_board.update_value_using_position("c2".to_string(), 1);
        sample_board.update_value_using_position("c7".to_string(), 6);
        sample_board.update_value_using_position("c8".to_string(), 5);
        sample_board.update_value_using_position("d1".to_string(), 5);
        sample_board.update_value_using_position("d3".to_string(), 9);
        sample_board.update_value_using_position("d5".to_string(), 3);
        sample_board.update_value_using_position("d7".to_string(), 7);
        sample_board.update_value_using_position("d8".to_string(), 8);
        sample_board.update_value_using_position("e5".to_string(), 7);
        sample_board.update_value_using_position("f2".to_string(), 4);
        sample_board.update_value_using_position("f3".to_string(), 8);
        sample_board.update_value_using_position("f5".to_string(), 2);
        sample_board.update_value_using_position("f7".to_string(), 1);
        sample_board.update_value_using_position("f9".to_string(), 3);
        sample_board.update_value_using_position("g2".to_string(), 5);
        sample_board.update_value_using_position("g3".to_string(), 2);
        sample_board.update_value_using_position("g8".to_string(), 9);
        sample_board.update_value_using_position("h3".to_string(), 1);
        sample_board.update_value_using_position("i1".to_string(), 3);
        sample_board.update_value_using_position("i4".to_string(), 9);
        sample_board.update_value_using_position("i6".to_string(), 2);
        sample_board.update_value_using_position("i9".to_string(), 5);
        sample_board
    }

    fn generate_sample_easy() -> Sudoku {
        let mut sample_board = Sudoku::new();
        sample_board.update_value_using_position("a1".to_string(), 4);
        sample_board.update_value_using_position("a3".to_string(), 1);
        sample_board.update_value_using_position("a4".to_string(), 2);
        sample_board.update_value_using_position("a5".to_string(), 9);
        sample_board.update_value_using_position("a8".to_string(), 7);
        sample_board.update_value_using_position("a9".to_string(), 5);
        sample_board.update_value_using_position("b1".to_string(), 2);
        sample_board.update_value_using_position("b4".to_string(), 3);
        sample_board.update_value_using_position("b7".to_string(), 8);
        sample_board.update_value_using_position("c2".to_string(), 7);
        sample_board.update_value_using_position("c5".to_string(), 8);
        sample_board.update_value_using_position("c9".to_string(), 6);
        sample_board.update_value_using_position("d4".to_string(), 1);
        sample_board.update_value_using_position("d6".to_string(), 3);
        sample_board.update_value_using_position("d8".to_string(), 6);
        sample_board.update_value_using_position("d9".to_string(), 2);
        sample_board.update_value_using_position("e1".to_string(), 1);
        sample_board.update_value_using_position("e3".to_string(), 5);
        sample_board.update_value_using_position("e7".to_string(), 4);
        sample_board.update_value_using_position("e9".to_string(), 3);
        sample_board.update_value_using_position("f1".to_string(), 7);
        sample_board.update_value_using_position("f2".to_string(), 3);
        sample_board.update_value_using_position("f4".to_string(), 6);
        sample_board.update_value_using_position("f6".to_string(), 8);
        sample_board.update_value_using_position("g1".to_string(), 6);
        sample_board.update_value_using_position("g5".to_string(), 2);
        sample_board.update_value_using_position("g8".to_string(), 3);
        sample_board.update_value_using_position("h3".to_string(), 7);
        sample_board.update_value_using_position("h6".to_string(), 1);
        sample_board.update_value_using_position("h9".to_string(), 4);
        sample_board.update_value_using_position("i1".to_string(), 8);
        sample_board.update_value_using_position("i2".to_string(), 9);
        sample_board.update_value_using_position("i5".to_string(), 6);
        sample_board.update_value_using_position("i6".to_string(), 5);
        sample_board.update_value_using_position("i7".to_string(), 1);
        sample_board.update_value_using_position("i9".to_string(), 7);
        sample_board
    }

    #[test]
    fn read_image_sudoku_easy() {
        let sample = generate_sample_easy();
        let mut reader = Reader::new("assets/sample_easy.jpg".to_string()).expect("Failed to create Reader for easy test");
        let result = reader.process_image();
        assert_eq!(sample, result);
    }

    #[test]
    fn read_image_sudoku_hard() {
        let sample = generate_sample_hard();
        let mut reader = Reader::new("assets/sample_hard.png".to_string()).expect("Failed to create Reader for hard test");
        let result = reader.process_image();
        assert_eq!(sample, result);
    }

    #[test]
    fn read_image_sudoku_very_hard() {
        let sample = generate_sample_very_hard();
        let mut reader = Reader::new("assets/sample_very_hard.png".to_string()).expect("Failed to create Reader for very_hard test");
        let result = reader.process_image();
        assert_eq!(sample, result);
    }

    #[test]
    fn reader_new_file_not_found() {
        let result = Reader::new("non_existent_file.png".to_string());
        assert!(result.is_err());
        match result.err().unwrap() {
            ReaderError::ImageOpenError(_) => { /* Expected error */ }
            // Add other ReaderError variants here if they become possible
        }
    }
}
