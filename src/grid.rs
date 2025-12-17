use std::{fmt, iter};

use crate::tile::{Tile, TileType};

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<TileType>>,
    pub column_x: Vec<u16>,
    pub row_y: Vec<u16>,
}

impl Grid {
    pub fn from_tiles(tiles: &[Tile]) -> Self {
        let mut column_x = tiles.iter().map(|tile| tile.grid_x).collect::<Vec<_>>();
        column_x.sort();
        column_x.dedup_by(|a, b| a.abs_diff(*b) <= 1);

        let mut row_y = tiles.iter().map(|tile| tile.grid_y).collect::<Vec<_>>();
        row_y.sort();
        row_y.dedup_by(|a, b| a.abs_diff(*b) <= 1);

        let mut cells = iter::repeat_with(|| {
            iter::repeat_n(TileType::Empty, column_x.len()).collect::<Vec<_>>()
        })
        .take(row_y.len())
        .collect::<Vec<_>>();

        for tile in tiles {
            let column = column_x
                .iter()
                .position(|x| x.abs_diff(tile.grid_x) <= 1)
                .unwrap();

            let row = row_y
                .iter()
                .position(|y| y.abs_diff(tile.grid_y) <= 1)
                .unwrap();

            cells[row][column] = tile.tile_type;
        }

        Self {
            cells,
            column_x,
            row_y,
        }
    }

    pub fn rows(&self) -> usize {
        self.row_y.len()
    }

    pub fn columns(&self) -> usize {
        self.column_x.len()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            self.cells
                .iter()
                .map(|row| row
                    .iter()
                    .map(|tile| format!("{}", tile.as_char()))
                    .collect::<Vec<_>>()
                    .join(" "))
                .collect::<Vec<_>>()
                .join("\n\n")
        )
    }
}
