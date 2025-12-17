mod bgra;
mod indexed;
pub mod png;
mod rgb;
pub mod x;

pub trait Image {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn pixel(&self, x: u32, y: u32) -> (u8, u8, u8);
}
