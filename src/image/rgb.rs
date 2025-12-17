use super::Image;

pub struct ImageRgb {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub line_size: usize,
}

impl Image for ImageRgb {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let pixel_index =
            self.line_size * usize::try_from(y).unwrap() + 3 * usize::try_from(x).unwrap();

        (
            self.bytes[pixel_index],
            self.bytes[pixel_index + 1],
            self.bytes[pixel_index + 2],
        )
    }
}
