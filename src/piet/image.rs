use piet::types;

pub struct Image {
    width: usize,
    height: usize,
    image: Vec<Vec<types::Color>>,
}

impl Image {
    pub fn new(vec: Vec<Vec<types::Color>>) -> Image {
        Image {
            width: vec.len(),
            height: vec[0].len(),
            image: vec,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> types::Color {
        self.image[x][y]
    }

    pub fn is_inner(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
