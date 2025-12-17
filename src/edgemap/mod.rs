mod internal;
mod printable;
mod trail;

use internal::EdgeMapInternal;

use crate::grid::Grid;

#[derive(Clone, Debug)]
pub struct EdgeMap {
    pub cells: Vec<Vec<OutgoingEdges>>,
}

// Edges to grid points further down the iteration order
#[derive(Clone, Copy, Debug)]
pub struct OutgoingEdges {
    pub r: Option<PathType>,
    pub dl: Option<PathType>,
    pub d: Option<PathType>,
    pub dr: Option<PathType>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathType {
    Triangle,
    Diamond,
    Square,
}

impl EdgeMap {
    pub fn find<F, T>(grid: &Grid, mut got_solution: F) -> Result<(), T>
    where
        F: FnMut(&EdgeMap) -> Result<(), T>,
    {
        EdgeMapInternal::find(grid, &mut got_solution)
    }
}
