use super::Image;

pub struct ImageIndexed {
    pub bytes: Vec<u8>,
    pub palette: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub line_size: usize,
}

impl Image for ImageIndexed {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let pixel_index =
            self.line_size * usize::try_from(y).unwrap() + usize::try_from(x).unwrap();

        let color_index = usize::from(self.bytes[pixel_index]);

        (
            self.palette[3 * color_index],
            self.palette[3 * color_index + 1],
            self.palette[3 * color_index + 2],
        )
    }
}
