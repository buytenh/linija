use crate::pixels::Pixels;

impl Pixels {
    pub fn segment_all(&mut self) -> Vec<Pixels> {
        let mut segments = Vec::new();

        while let Some((x, ys)) = self.pixels.first_key_value() {
            let y = ys.first().unwrap();

            segments.push(self.segment(*x, *y));
        }

        segments
    }

    fn segment(&mut self, x: u16, y: u16) -> Pixels {
        let mut segment = Pixels::new();

        let mut expand_set = Pixels::new();

        assert!(self.remove(x, y));
        expand_set.insert(x, y);

        while let Some((x, y)) = expand_set.pop_first() {
            assert!(segment.insert(x, y));

            let neighbors = {
                let mut neighbors = Vec::with_capacity(4);

                if x > 0 {
                    neighbors.push((x - 1, y));
                }

                if x < u16::MAX {
                    neighbors.push((x + 1, y));
                }

                if y > 0 {
                    neighbors.push((x, y - 1));
                }

                if y < u16::MAX {
                    neighbors.push((x, y + 1));
                }

                neighbors
            };

            for (x, y) in neighbors {
                if self.remove(x, y) {
                    expand_set.insert(x, y);
                }
            }
        }

        segment
    }

    fn remove(&mut self, x: u16, y: u16) -> bool {
        if let Some(ys) = self.pixels.get_mut(&x)
            && ys.remove(&y)
        {
            if ys.is_empty() {
                self.pixels.remove(&x);
            }

            // self.bounds is now invalid

            true
        } else {
            false
        }
    }

    fn pop_first(&mut self) -> Option<(u16, u16)> {
        if let Some((x, ys)) = self.pixels.first_key_value() {
            let x = *x;
            let y = *ys.first().unwrap();

            self.remove(x, y);

            Some((x, y))
        } else {
            None
        }
    }

    pub fn num_holes(&self) -> usize {
        let mut invert = Pixels::new();

        for x in self.bounds.min_x..=self.bounds.max_x {
            for y in self.bounds.min_y..=self.bounds.max_y {
                if !self.contains(x, y) {
                    invert.insert(x, y);
                }
            }
        }

        invert.segment_all().len()
    }

    fn contains(&self, x: u16, y: u16) -> bool {
        if let Some(ys) = self.pixels.get(&x)
            && ys.contains(&y)
        {
            true
        } else {
            false
        }
    }
}
