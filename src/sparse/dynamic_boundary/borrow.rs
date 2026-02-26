use crate::sparse::dynamic_boundary::*;

pub struct BorrowBoundaryPiece<'a, const GRID_DIMENSION: usize> {
    inside: &'a CoordSet<GRID_DIMENSION>,
    outside: &'a CoordSet<GRID_DIMENSION>,
}

impl<'a, const GRID_DIMENSION: usize> BorrowBoundaryPiece<'a, GRID_DIMENSION> {
    pub fn new(
        inside: &'a CoordSet<GRID_DIMENSION>,
        outside: &'a CoordSet<GRID_DIMENSION>,
    ) -> Self {
        Self { inside, outside }
    }
}

impl<'a, const GRID_DIMENSION: usize> BoundaryPiece<GRID_DIMENSION>
    for BorrowBoundaryPiece<'a, GRID_DIMENSION>
{
    fn inside(&self) -> &CoordSet<GRID_DIMENSION> {
        self.inside
    }

    fn outside(&self) -> &CoordSet<GRID_DIMENSION> {
        self.outside
    }

    fn to_owned(self) -> OwnedBoundaryPiece<GRID_DIMENSION> {
        OwnedBoundaryPiece::new(self.inside.clone(), self.outside.clone())
    }
}
