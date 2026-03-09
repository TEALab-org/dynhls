use crate::sparse::dynamic_boundary::*;
use crate::util::*;

pub struct StencilPositionResult<const GRID_DIMENSION: usize> {
    pub center: Coord<GRID_DIMENSION>,
    pub new_cells: CoordSet<GRID_DIMENSION>,
}
