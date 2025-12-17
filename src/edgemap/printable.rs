use crate::{
    edgemap::{EdgeMap, PathType},
    grid::Grid,
};

impl EdgeMap {
    pub fn printable<'a>(&'a self, grid: &'a Grid) -> EdgeMapPrintable<'a> {
        EdgeMapPrintable {
            grid,
            edgemap: self,
        }
    }
}

pub struct EdgeMapPrintable<'a> {
    grid: &'a Grid,
    edgemap: &'a EdgeMap,
}

impl<'a> std::fmt::Display for EdgeMapPrintable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in 0..self.grid.rows() {
            for pt in [PathType::Triangle, PathType::Diamond, PathType::Square] {
                for column in 0..self.grid.columns() {
                    write!(f, "{}", self.grid.cells[row][column].as_char())?;

                    if column + 1 < self.grid.columns() {
                        if self.edgemap.cells[row][column].r == Some(pt) {
                            write!(f, "-")?;
                        } else {
                            write!(f, " ")?;
                        }
                    }
                }

                if pt != PathType::Square {
                    write!(f, "       ")?;
                }
            }

            if row + 1 < self.grid.rows() {
                writeln!(f)?;

                for pt in [PathType::Triangle, PathType::Diamond, PathType::Square] {
                    for column in 0..self.grid.columns() {
                        if self.edgemap.cells[row][column].d == Some(pt) {
                            write!(f, "|")?;
                        } else {
                            write!(f, " ")?;
                        }

                        if column + 1 < self.grid.columns() {
                            if self.edgemap.cells[row][column].dr == Some(pt) {
                                write!(f, "\\")?;
                            } else if self.edgemap.cells[row][column + 1].dl == Some(pt) {
                                write!(f, "/")?;
                            } else {
                                write!(f, " ")?;
                            }
                        }
                    }

                    if pt != PathType::Square {
                        write!(f, "       ")?;
                    }
                }

                writeln!(f)?;
            }
        }

        Ok(())
    }
}
