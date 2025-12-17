use x11rb::{
    protocol::xproto::{ConnectionExt, ImageFormat},
    rust_connection::RustConnection,
};

use super::{Image, bgra::ImageBgra};

pub fn from_window(conn: &RustConnection, window: u32) -> Result<Box<dyn Image>, String> {
    let geometry = conn
        .get_geometry(window)
        .unwrap()
        .reply()
        .map_err(|_| format!("Error retrieving geometry for window {:08x}", window))?;

    let width = geometry.width;
    let height = geometry.height;

    let image = conn
        .get_image(
            ImageFormat::Z_PIXMAP,
            window,
            0,
            0,
            width,
            height,
            0xffffffff,
        )
        .unwrap()
        .reply()
        .map_err(|_| format!("Error retrieving contents of window {:08x}", window))?;

    Ok(Box::new(ImageBgra {
        bytes: image.data,
        width: u32::from(width),
        height: u32::from(height),
        line_size: 4 * usize::from(width),
    }))
}
