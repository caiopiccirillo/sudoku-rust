use image;
pub struct Reader {
    pub img: image::DynamicImage,
}

impl Reader {
    pub fn new(path: String) -> Self {
        Self {
            img: image::open(path).unwrap(),
        }
    }
    pub fn read_image(&mut self,path: String){
        self.img = image::open(path).unwrap()
    }
}
