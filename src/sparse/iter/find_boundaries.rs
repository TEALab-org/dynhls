use crate::{sparse::dynamic_boundary::*, util::*};

pub struct OneBallBoundary<const GRID_DIMENSION: usize> {
    pub inside: CoordSet<GRID_DIMENSION>,
    pub outside: CoordSet<GRID_DIMENSION>,
}

pub fn initial_oneball_boundary<const GRID_DIMENSION: usize>(
    domain: &CoordSet<GRID_DIMENSION>,
) -> OneBallBoundary<GRID_DIMENSION> {
    let mut inside = CoordSet::empty();
    let mut outside = CoordSet::empty();

    for coord in domain.coord_iter() {
        for offset in AABB::one_ball_iter() {
            let dep = coord + offset;
            if !domain.contains(&dep) {
                inside.add(*coord);
                outside.add(dep);
            }
        }
    }

    OneBallBoundary { inside, outside }
}
