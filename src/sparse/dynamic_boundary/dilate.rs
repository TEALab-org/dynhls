use crate::sparse::dynamic_boundary::*;
use crate::stencil::*;

pub fn dilate_in<
    const GRID_DIMENSION: usize,
    const NEIGHBORHOOD_SIZE: usize,
    InputBoundType: BoundaryPiece<GRID_DIMENSION>,
    StencilType: TVStencil<GRID_DIMENSION, NEIGHBORHOOD_SIZE>,
>(
    boundary: &InputBoundType,
    stencil: &StencilType,
) -> OwnedBoundaryPiece<GRID_DIMENSION> {
    // Old in becomes out, calc new in?

    let mut new_in = CoordSet::empty();

    for coord in boundary.inside().coord_iter() {
        for offset in stencil.roi_offsets() {
            let n_coord = coord + offset;
            if !boundary.inside().contains(&n_coord)
                && !boundary.outside().contains(&n_coord)
            {
                new_in.add(n_coord);
            }
        }
    }

    OwnedBoundaryPiece::new(new_in, boundary.inside().clone())
}

pub fn dilate_dynamic_in<
    const GRID_DIMENSION: usize,
    const NEIGHBORHOOD_SIZE: usize,
    StencilType: TVStencil<GRID_DIMENSION, NEIGHBORHOOD_SIZE>,
>(
    boundary: &DynamicBoundary<GRID_DIMENSION>,
    stencil: &StencilType,
) -> DynamicBoundary<GRID_DIMENSION> {
    // Old in becomes out, calc new in?

    let mut new_in = CoordSet::empty();

    for coord in boundary.dilation_front.inside().coord_iter() {
        for offset in stencil.roi_offsets() {
            let n_coord = coord + offset;
            if !boundary.dilation_front.inside().contains(&n_coord)
                && !boundary.dilation_front.outside().contains(&n_coord)
                && !boundary.static_front.outside().contains(&n_coord)
            {
                new_in.add(n_coord);
            }
        }
    }

    let dilation_front = OwnedBoundaryPiece::new(
        new_in.clone(),
        boundary.dilation_front.inside().clone(),
    );
    let new_sf_in = boundary.static_front.inside().remove(&new_in);
    let static_front = OwnedBoundaryPiece::new(
        new_sf_in,
        boundary.static_front.outside().clone(),
    );
    DynamicBoundary::new(dilation_front, static_front)
}
