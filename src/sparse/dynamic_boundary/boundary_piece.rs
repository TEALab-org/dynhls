use crate::sparse::dynamic_boundary::*;

pub trait BoundaryPiece<const GRID_DIMENSION: usize> {
    fn inside(&self) -> &CoordSet<GRID_DIMENSION>;
    fn outside(&self) -> &CoordSet<GRID_DIMENSION>;
    fn to_owned(self) -> OwnedBoundaryPiece<GRID_DIMENSION>;
}
