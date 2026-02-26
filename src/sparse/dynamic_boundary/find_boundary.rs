use crate::sparse::dynamic_boundary::*;
use crate::stencil::*;
use crate::util::*;

pub fn find_region_boundaries<
    const GRID_DIMENSION: usize,
    const NEIGHBORHOOD_SIZE: usize,
    StencilType: TVStencil<GRID_DIMENSION, NEIGHBORHOOD_SIZE>,
>(
    region: &CoordSet<GRID_DIMENSION>,
    stencil: &StencilType,
) -> OwnedBoundaryPiece<GRID_DIMENSION> {
    let mut inside = CoordSet::empty();
    let mut outside = CoordSet::empty();

    for coord in region.coord_iter() {
        for offset in stencil.offsets() {
            let n_coord = coord + offset;
            if !region.contains(&n_coord) {
                outside.add(n_coord);
            }
        }
    }

    for coord in outside.coord_iter() {
        for offset in stencil.roi_offsets() {
            let n_coord = coord + offset;
            if region.contains(&n_coord) {
                inside.add(n_coord);
            }
        }
    }

    OwnedBoundaryPiece::new(inside, outside)
}

pub fn find_region_boundaries_static_rad<
    const GRID_DIMENSION: usize,
    const NEIGHBORHOOD_SIZE: usize,
    StencilType: TVStencil<GRID_DIMENSION, NEIGHBORHOOD_SIZE>,
>(
    region: &CoordSet<GRID_DIMENSION>,
    stencil: &StencilType,
) -> OwnedBoundaryPiece<GRID_DIMENSION> {
    let mut inside = CoordSet::empty();
    let mut outside = CoordSet::empty();
    let radius = stencil.radius();
    let radius_coord = Coord::from_element(radius);
    let static_offsets = AABB::from_mm(-radius_coord, radius_coord);

    for coord in region.coord_iter() {
        for offset in static_offsets.coord_iter() {
            let n_coord = coord + offset;
            if !region.contains(&n_coord) {
                outside.add(n_coord);
            }
        }
    }

    for coord in outside.coord_iter() {
        for offset in static_offsets.coord_iter() {
            let n_coord = coord + offset;
            if region.contains(&n_coord) {
                inside.add(n_coord);
            }
        }
    }

    OwnedBoundaryPiece::new(inside, outside)
}
