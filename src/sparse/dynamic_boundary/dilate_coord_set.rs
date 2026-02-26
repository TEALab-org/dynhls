use crate::sparse::dynamic_boundary::*;
use crate::stencil::*;

pub fn dilate_in_coord_set<
    const GRID_DIMENSION: usize,
    const NEIGHBORHOOD_SIZE: usize,
    StencilType: TVStencil<GRID_DIMENSION, NEIGHBORHOOD_SIZE>,
>(
    region: &CoordSet<GRID_DIMENSION>,
    stencil: &StencilType,
) -> CoordSet<GRID_DIMENSION> {
    // Old in becomes out, calc new in?

    let mut candidates = CoordSet::empty();
    for coord in region.coord_iter() {
        for offset in stencil.roi_offsets() {
            let n_coord = coord + offset;
            candidates.add(n_coord);
        }
    }

    let mut result = CoordSet::empty();
    for coord in candidates.coord_iter() {
        let mut add_flag = true;
        for offset in stencil.offsets() {
            let n_coord = coord + offset;
            if !region.contains(&n_coord) {
                add_flag = false;
                break;
            }
        }
        if add_flag {
            result.add(*coord);
        }
    }

    result
}
