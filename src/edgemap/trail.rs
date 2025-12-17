use crate::{
    edgemap::{EdgeMap, PathType},
    grid::Grid,
    trail::EdgeSet,
};

impl EdgeMap {
    pub fn trail(&self, grid: &Grid, pt: PathType) -> Option<Vec<(usize, usize)>> {
        let mut edgeset = EdgeSet::new();

        for row in 0..grid.rows() {
            for column in 0..grid.columns() {
                if self.cells[row][column].r == Some(pt) {
                    edgeset.add_edge((row, column), (row, column + 1));
                }
                if self.cells[row][column].dl == Some(pt) {
                    edgeset.add_edge((row, column), (row + 1, column - 1));
                }
                if self.cells[row][column].d == Some(pt) {
                    edgeset.add_edge((row, column), (row + 1, column));
                }
                if self.cells[row][column].dr == Some(pt) {
                    edgeset.add_edge((row, column), (row + 1, column + 1));
                }
            }
        }

        edgeset.trace_trail()
    }
}
