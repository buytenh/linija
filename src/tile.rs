use crate::{bounds::Bounds, color_scheme::determine_color_scheme, image::Image, pixels::Pixels};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tile {
    pub grid_x: u16,
    pub grid_y: u16,
    pub tile_type: TileType,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TileType {
    Empty,
    TriangleTerminal,
    Triangle,
    DiamondTerminal,
    Diamond,
    SquareTerminal,
    Square,
    Connect2,
    Connect3,
    Connect4,
}

impl Tile {
    pub fn new(bounds: &Bounds, tile_type: TileType) -> Self {
        let grid_x = (bounds.min_x + bounds.max_x) / 2;

        let grid_y = if tile_type == TileType::TriangleTerminal || tile_type == TileType::Triangle {
            (bounds.min_y + 2 * bounds.max_y) / 3
        } else {
            (bounds.min_y + bounds.max_y) / 2
        };

        Tile {
            grid_x,
            grid_y,
            tile_type,
        }
    }

    pub fn detect_tiles<T: Image + ?Sized>(image: &T) -> Result<Vec<Self>, String> {
        let mut triangle_outer = Pixels::new();
        let mut triangle = Pixels::new();
        let mut diamond_outer = Pixels::new();
        let mut diamond = Pixels::new();
        let mut square_outer = Pixels::new();
        let mut square = Pixels::new();
        let mut connector = Pixels::new();

        let color_scheme = {
            let p = image.pixel(0, 0);

            determine_color_scheme(&[p.0, p.1, p.2])
        }
        .ok_or_else(|| "Can't determine color scheme for image".to_string())?;

        for y in 0..image.height() {
            for x in 0..image.width() {
                let p = image.pixel(x, y);

                let pixel = [p.0, p.1, p.2];

                if pixel == color_scheme.triangle_outer_color {
                    triangle_outer.insert(x, y);
                } else if pixel == color_scheme.triangle_color {
                    triangle.insert(x, y);
                } else if pixel == color_scheme.diamond_outer_color {
                    diamond_outer.insert(x, y);
                } else if pixel == color_scheme.diamond_color {
                    diamond.insert(x, y);
                } else if pixel == color_scheme.square_outer_color {
                    square_outer.insert(x, y);
                } else if pixel == color_scheme.square_color {
                    square.insert(x, y);
                } else if pixel == color_scheme.connector_color {
                    connector.insert(x, y);
                }
            }
        }

        let mut tiles = Vec::new();

        {
            let triangle_outer = triangle_outer.segment_all();
            let triangle = triangle.segment_all();

            if triangle.is_empty() {
                assert!(triangle_outer.is_empty());
            } else {
                assert_eq!(triangle_outer.len(), 2);
            }

            for segment in &triangle_outer {
                tiles.push(Tile::new(&segment.bounds, TileType::TriangleTerminal));
            }

            for segment in &triangle {
                if !segment.is_within(&triangle_outer[0]) && !segment.is_within(&triangle_outer[1])
                {
                    tiles.push(Tile::new(&segment.bounds, TileType::Triangle));
                }
            }
        }

        {
            let diamond_outer = diamond_outer.segment_all();
            let diamond = diamond.segment_all();

            if diamond.is_empty() {
                assert!(diamond_outer.is_empty());
            } else {
                assert_eq!(diamond_outer.len(), 2);
            }

            for segment in &diamond_outer {
                tiles.push(Tile::new(&segment.bounds, TileType::DiamondTerminal));
            }

            for segment in &diamond {
                if !segment.is_within(&diamond_outer[0]) && !segment.is_within(&diamond_outer[1]) {
                    tiles.push(Tile::new(&segment.bounds, TileType::Diamond));
                }
            }
        }

        {
            let square_outer = square_outer.segment_all();
            let square = square.segment_all();

            if square.is_empty() {
                assert!(square_outer.is_empty());
            } else {
                assert_eq!(square_outer.len(), 2);
            }

            for segment in &square_outer {
                tiles.push(Tile::new(&segment.bounds, TileType::SquareTerminal));
            }

            for segment in &square {
                if !segment.is_within(&square_outer[0]) && !segment.is_within(&square_outer[1]) {
                    tiles.push(Tile::new(&segment.bounds, TileType::Square));
                }
            }
        }

        for segment in &connector.segment_all() {
            let num_holes = segment.num_holes();

            tiles.push(Tile::new(
                &segment.bounds,
                match num_holes {
                    6 => TileType::Connect2,
                    7 => TileType::Connect3,
                    8 => TileType::Connect4,
                    _ => panic!(),
                },
            ));
        }

        tiles.sort();

        Ok(tiles)
    }
}

impl TileType {
    pub fn is_triangle(&self) -> bool {
        *self == TileType::TriangleTerminal || *self == TileType::Triangle
    }

    pub fn is_diamond(&self) -> bool {
        *self == TileType::DiamondTerminal || *self == TileType::Diamond
    }

    pub fn is_square(&self) -> bool {
        *self == TileType::SquareTerminal || *self == TileType::Square
    }

    pub fn is_connector(&self) -> bool {
        *self == TileType::Connect2 || *self == TileType::Connect3 || *self == TileType::Connect4
    }

    pub fn as_char(&self) -> char {
        match self {
            TileType::Empty => ' ',
            TileType::TriangleTerminal => 'T',
            TileType::Triangle => 't',
            TileType::DiamondTerminal => 'D',
            TileType::Diamond => 'd',
            TileType::SquareTerminal => 'S',
            TileType::Square => 's',
            TileType::Connect2 => '2',
            TileType::Connect3 => '3',
            TileType::Connect4 => '4',
        }
    }
}
