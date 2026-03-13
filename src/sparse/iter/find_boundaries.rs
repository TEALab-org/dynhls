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

pub fn fill_boundary<const GRID_DIMENSION: usize>(
    boundary: &OneBallBoundary<GRID_DIMENSION>
) -> CoordSet<GRID_DIMENSION> {
    let mut result = CoordSet::empty();
    let mut new_points = boundary.inside.clone();
    let mut next_points = CoordSet::empty();

    while !new_points.is_empty() {
        for coord in new_points.coord_iter() {
            result.add(*coord);
            for offset in AABB::one_ball_iter() {
                let n_coord = coord + offset;
                if !boundary.outside.contains(&n_coord) 
                && !result.contains(&n_coord) {
                    next_points.add(n_coord);
                }
            }
        }
        
        std::mem::swap(&mut new_points, &mut next_points);
        next_points.clear();

        println!("result: {}", result.len());
    }

    result
}
