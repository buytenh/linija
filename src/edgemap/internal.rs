use std::iter;

use crate::{
    edgemap::{EdgeMap, OutgoingEdges, PathType},
    grid::Grid,
    tile::TileType,
};

#[derive(Debug)]
pub struct EdgeMapInternal<'a> {
    grid: &'a Grid,
    edgemap: EdgeMap,
}

impl<'a> EdgeMapInternal<'a> {
    pub fn find<F, T>(grid: &'a Grid, got_solution: &mut F) -> Result<(), T>
    where
        F: FnMut(&EdgeMap) -> Result<(), T>,
    {
        Self::new(grid).find_from_cell(0, 0, got_solution)
    }

    fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            edgemap: EdgeMap {
                cells: iter::repeat_with(|| {
                    iter::repeat_n(
                        OutgoingEdges {
                            r: None,
                            dl: None,
                            d: None,
                            dr: None,
                        },
                        grid.columns(),
                    )
                    .collect::<Vec<_>>()
                })
                .take(grid.rows())
                .collect::<Vec<_>>(),
            },
        }
    }

    fn find_from_cell<F, T>(
        &mut self,
        row: usize,
        column: usize,
        got_solution: &mut F,
    ) -> Result<(), T>
    where
        F: FnMut(&EdgeMap) -> Result<(), T>,
    {
        let degree_remaining = self.degree_remaining(row, column);

        let mut try_combinations = Vec::with_capacity(6);

        match degree_remaining {
            0 => try_combinations.push((false, false, false, false)),
            1 => {
                try_combinations.push((true, false, false, false));
                try_combinations.push((false, true, false, false));
                try_combinations.push((false, false, true, false));
                try_combinations.push((false, false, false, true));
            }
            2 => {
                try_combinations.push((true, true, false, false));
                try_combinations.push((true, false, true, false));
                try_combinations.push((true, false, false, true));
                try_combinations.push((false, true, true, false));
                try_combinations.push((false, true, false, true));
                try_combinations.push((false, false, true, true));
            }
            3 => {
                try_combinations.push((true, true, true, false));
                try_combinations.push((true, true, false, true));
                try_combinations.push((true, false, true, true));
                try_combinations.push((false, true, true, true));
            }
            4 => {
                try_combinations.push((true, true, true, true));
            }
            5..=8 => {}
            _ => panic!(),
        }

        if !try_combinations.is_empty() {
            let options_r = if column < self.columns() - 1 {
                self.connection_options(row, column, row, column + 1)
            } else {
                &[]
            };

            let options_dl = if (row < self.rows() - 1 && column > 0)
                && self.edgemap.cells[row][column - 1].dr.is_none()
            {
                self.connection_options(row, column, row + 1, column - 1)
            } else {
                &[]
            };

            let options_d = if row < self.rows() - 1 {
                self.connection_options(row, column, row + 1, column)
            } else {
                &[]
            };

            let options_dr = if row < self.rows() - 1 && column < self.columns() - 1 {
                self.connection_options(row, column, row + 1, column + 1)
            } else {
                &[]
            };

            for (r, dl, d, dr) in try_combinations {
                let options_r = if r { options_r } else { &[None] };
                let options_dl = if dl { options_dl } else { &[None] };
                let options_d = if d { options_d } else { &[None] };
                let options_dr = if dr { options_dr } else { &[None] };

                for r in options_r {
                    self.edgemap.cells[row][column].r = *r;

                    for dl in options_dl {
                        self.edgemap.cells[row][column].dl = *dl;

                        for d in options_d {
                            self.edgemap.cells[row][column].d = *d;

                            for dr in options_dr {
                                self.edgemap.cells[row][column].dr = *dr;

                                if self.check_degree_satisfied(row, column) {
                                    if column + 1 < self.columns() {
                                        self.find_from_cell(row, column + 1, got_solution)?;
                                    } else if row + 1 < self.rows() {
                                        self.find_from_cell(row + 1, 0, got_solution)?;
                                    } else {
                                        got_solution(&self.edgemap)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // TODO: consider getting rid of this and taking inbound direction into
            // account when evaluating remaining degree in ::connection_options()
            self.edgemap.cells[row][column].r = None;
            self.edgemap.cells[row][column].dl = None;
            self.edgemap.cells[row][column].d = None;
            self.edgemap.cells[row][column].dr = None;
        }

        Ok(())
    }

    fn degree_remaining(&self, row: usize, column: usize) -> usize {
        self.total_degree(row, column)
            .strict_sub(self.current_in_degree(row, column))
    }

    fn total_degree(&self, row: usize, column: usize) -> usize {
        match self.grid.cells[row][column] {
            TileType::Empty => 0,
            TileType::TriangleTerminal => 1,
            TileType::Triangle => 2,
            TileType::DiamondTerminal => 1,
            TileType::Diamond => 2,
            TileType::SquareTerminal => 1,
            TileType::Square => 2,
            TileType::Connect2 => 4,
            TileType::Connect3 => 6,
            TileType::Connect4 => 8,
        }
    }

    fn current_in_degree(&self, row: usize, column: usize) -> usize {
        let in_degrees = self.current_in_degrees(row, column);

        in_degrees.0 + in_degrees.1 + in_degrees.2
    }

    fn current_in_degrees(&self, row: usize, column: usize) -> (usize, usize, usize) {
        let mut path_types = Vec::with_capacity(4);

        if column > 0 {
            path_types.push(self.edgemap.cells[row][column - 1].r);
        }

        if row > 0 {
            if column + 1 < self.columns() {
                path_types.push(self.edgemap.cells[row - 1][column + 1].dl);
            }

            path_types.push(self.edgemap.cells[row - 1][column].d);

            if column > 0 {
                path_types.push(self.edgemap.cells[row - 1][column - 1].dr);
            }
        }

        let mut triangle_degree = 0;
        let mut diamond_degree = 0;
        let mut square_degree = 0;

        for path_type in path_types {
            match path_type {
                None => {}
                Some(PathType::Triangle) => triangle_degree += 1,
                Some(PathType::Diamond) => diamond_degree += 1,
                Some(PathType::Square) => square_degree += 1,
            }
        }

        (triangle_degree, diamond_degree, square_degree)
    }

    fn rows(&self) -> usize {
        self.grid.rows()
    }

    fn columns(&self) -> usize {
        self.grid.columns()
    }

    fn connection_options(
        &self,
        row_a: usize,
        column_a: usize,
        row_b: usize,
        column_b: usize,
    ) -> &'static [Option<PathType>] {
        if self.degree_remaining(row_b, column_b) > 0 {
            let path_type_a = self.grid.cells[row_a][column_a];
            let path_type_b = self.grid.cells[row_b][column_b];

            if path_type_a.is_connector() && path_type_b.is_connector() {
                &[
                    Some(PathType::Triangle),
                    Some(PathType::Diamond),
                    Some(PathType::Square),
                ]
            } else if (path_type_a.is_triangle() || path_type_a.is_connector())
                && (path_type_b.is_triangle() || path_type_b.is_connector())
            {
                &[Some(PathType::Triangle)]
            } else if (path_type_a.is_diamond() || path_type_a.is_connector())
                && (path_type_b.is_diamond() || path_type_b.is_connector())
            {
                &[Some(PathType::Diamond)]
            } else if (path_type_a.is_square() || path_type_a.is_connector())
                && (path_type_b.is_square() || path_type_b.is_connector())
            {
                &[Some(PathType::Square)]
            } else {
                &[]
            }
        } else {
            &[]
        }
    }

    fn check_degree_satisfied(&self, row: usize, column: usize) -> bool {
        let in_degrees = self.current_in_degrees(row, column);
        let out_degrees = self.current_out_degrees(row, column);

        let triangle_degree = in_degrees.0 + out_degrees.0;
        let diamond_degree = in_degrees.1 + out_degrees.1;
        let square_degree = in_degrees.2 + out_degrees.2;

        match self.grid.cells[row][column] {
            TileType::Empty => {
                assert_eq!(triangle_degree, 0);
                assert_eq!(diamond_degree, 0);
                assert_eq!(square_degree, 0);

                true
            }
            TileType::TriangleTerminal => {
                assert_eq!(diamond_degree, 0);
                assert_eq!(square_degree, 0);

                triangle_degree == 1
            }
            TileType::Triangle => {
                assert_eq!(diamond_degree, 0);
                assert_eq!(square_degree, 0);

                triangle_degree == 2
            }
            TileType::DiamondTerminal => {
                assert_eq!(triangle_degree, 0);
                assert_eq!(square_degree, 0);

                diamond_degree == 1
            }
            TileType::Diamond => {
                assert_eq!(triangle_degree, 0);
                assert_eq!(square_degree, 0);

                diamond_degree == 2
            }
            TileType::SquareTerminal => {
                assert_eq!(triangle_degree, 0);
                assert_eq!(diamond_degree, 0);

                square_degree == 1
            }
            TileType::Square => {
                assert_eq!(triangle_degree, 0);
                assert_eq!(diamond_degree, 0);

                square_degree == 2
            }
            TileType::Connect2 => {
                triangle_degree + diamond_degree + square_degree == 4
                    && (triangle_degree & 1) == 0
                    && (diamond_degree & 1) == 0
                    && (square_degree & 1) == 0
            }
            TileType::Connect3 => {
                triangle_degree + diamond_degree + square_degree == 6
                    && (triangle_degree & 1) == 0
                    && (diamond_degree & 1) == 0
                    && (square_degree & 1) == 0
            }
            TileType::Connect4 => {
                triangle_degree + diamond_degree + square_degree == 8
                    && (triangle_degree & 1) == 0
                    && (diamond_degree & 1) == 0
                    && (square_degree & 1) == 0
            }
        }
    }

    fn current_out_degrees(&self, row: usize, column: usize) -> (usize, usize, usize) {
        let cell = &self.edgemap.cells[row][column];

        let mut triangle_degree = 0;
        let mut diamond_degree = 0;
        let mut square_degree = 0;

        for path_type in [cell.r, cell.dl, cell.d, cell.dr] {
            match path_type {
                None => {}
                Some(PathType::Triangle) => triangle_degree += 1,
                Some(PathType::Diamond) => diamond_degree += 1,
                Some(PathType::Square) => square_degree += 1,
            }
        }

        (triangle_degree, diamond_degree, square_degree)
    }
}
