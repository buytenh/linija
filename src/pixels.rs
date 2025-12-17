use std::collections::{BTreeMap, BTreeSet};

use crate::bounds::Bounds;

#[derive(Debug, Default)]
pub struct Pixels {
    pub pixels: BTreeMap<u16, BTreeSet<u16>>,
    pub bounds: Bounds,
}

impl Pixels {
    pub fn new() -> Self {
        Self {
            pixels: BTreeMap::new(),
            bounds: Bounds::new(),
        }
    }

    pub fn insert<T>(&mut self, x: T, y: T) -> bool
    where
        T: TryInto<u16>,
        <T as TryInto<u16>>::Error: std::fmt::Debug,
    {
        let x = x.try_into().unwrap();
        let y = y.try_into().unwrap();

        let inserted = self.pixels.entry(x).or_default().insert(y);

        if inserted {
            self.bounds.update(x, y);
        }

        inserted
    }

    pub fn is_within(&self, other: &Self) -> bool {
        self.bounds.is_within(&other.bounds)
    }

    /*
    pub fn write_png(&self, file: &str, width: u32, height: u32) {
        use std::{fs::File, io::BufWriter};

        use png::{BitDepth, ColorType, Encoder};

        assert!(width <= u32::from(u16::MAX));
        assert!(height <= u32::from(u16::MAX));

        let mut encoder = Encoder::new(BufWriter::new(File::create(file).unwrap()), width, height);

        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        let mut bytes = Vec::with_capacity(usize::try_from(3 * width * height).unwrap());

        for y in 0..u16::try_from(height).unwrap() {
            for x in 0..u16::try_from(width).unwrap() {
                if let Some(ys) = self.pixels.get(&x)
                    && ys.contains(&y)
                {
                    bytes.extend(&[255, 255, 255]);
                } else {
                    bytes.extend(&[0, 0, 0]);
                }
            }
        }

        writer.write_image_data(&bytes).unwrap();
    }
    */
}
