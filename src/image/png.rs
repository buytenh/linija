use std::{fs::File, io::BufReader};

use png::{BitDepth, ColorType, Decoder};

use super::{Image, indexed::ImageIndexed, rgb::ImageRgb};

pub fn open(file: &str) -> Result<Box<dyn Image>, String> {
    let mut reader = Decoder::new(BufReader::new(File::open(file).unwrap()))
        .read_info()
        .unwrap();

    let mut buf = vec![0; reader.output_buffer_size().unwrap()];

    let output_info = reader.next_frame(&mut buf).unwrap();

    buf.truncate(output_info.buffer_size());

    if output_info.bit_depth == BitDepth::Eight {
        if output_info.color_type == ColorType::Rgb {
            assert_eq!(
                output_info.line_size,
                3 * usize::try_from(output_info.width).unwrap()
            );

            Ok(Box::new(ImageRgb {
                bytes: buf,
                width: output_info.width,
                height: output_info.height,
                line_size: output_info.line_size,
            }))
        } else if output_info.color_type == ColorType::Indexed {
            assert_eq!(
                output_info.line_size,
                usize::try_from(output_info.width).unwrap()
            );

            Ok(Box::new(ImageIndexed {
                bytes: buf,
                palette: reader.info().palette.as_ref().unwrap().to_vec(),
                width: output_info.width,
                height: output_info.height,
                line_size: output_info.line_size,
            }))
        } else {
            Err(format!("Unknown color type {:?}", output_info.color_type))
        }
    } else {
        Err(format!(
            "Unknown color type {:?} and/or bit depth {:?}",
            output_info.color_type, output_info.bit_depth
        ))
    }
}
