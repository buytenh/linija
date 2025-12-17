#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Bounds {
    pub min_x: u16,
    pub max_x: u16,
    pub min_y: u16,
    pub max_y: u16,
}

impl Bounds {
    pub fn new() -> Self {
        Bounds {
            min_x: u16::MAX,
            max_x: u16::MIN,
            min_y: u16::MAX,
            max_y: u16::MIN,
        }
    }

    pub fn update(&mut self, x: u16, y: u16) {
        if x < self.min_x {
            self.min_x = x;
        }

        if x > self.max_x {
            self.max_x = x;
        }

        if y < self.min_y {
            self.min_y = y;
        }

        if y > self.max_y {
            self.max_y = y;
        }
    }

    pub fn is_within(&self, other: &Self) -> bool {
        self.min_x > other.min_x
            && self.max_x < other.max_x
            && self.min_y > other.min_y
            && self.max_y < other.max_y
    }
}
