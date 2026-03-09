use crate::sparse::dynamic_boundary::*;
use crate::stencil::*;
use crate::util::*;
use std::collections::HashSet;

pub struct Explorer<
    'a,
    const GRID_DIMENSION: usize,
    const NEIGHBORHOOD_SIZE: usize,
    StencilType: TVStencil<GRID_DIMENSION, NEIGHBORHOOD_SIZE>,
> {
    stencil: &'a StencilType,
    inside: CoordMap<GRID_DIMENSION, bool>,
    outside: CoordSet<GRID_DIMENSION>,
    pub new_inside: CoordSet<GRID_DIMENSION>,
    pub new_outside: CoordSet<GRID_DIMENSION>,
    pub new_explored: CoordSet<GRID_DIMENSION>,
    pub front: HashSet<Coord<GRID_DIMENSION>>,
}

// So there's the evaluated region,
// Then there's the frontier we need to add to
// whenever we move the stencil

// For now lets trace the boundary with 1-1 stencil
impl<
        'a,
        const GRID_DIMENSION: usize,
        const NEIGHBORHOOD_SIZE: usize,
        StencilType: TVStencil<GRID_DIMENSION, NEIGHBORHOOD_SIZE>,
    > Explorer<'a, GRID_DIMENSION, NEIGHBORHOOD_SIZE, StencilType>
{
    pub fn new(
        stencil: &'a StencilType,
        inside_set: CoordSet<GRID_DIMENSION>,
        outside: CoordSet<GRID_DIMENSION>,
    ) -> Self {
        let inside = CoordMap::from_coord_set(&inside_set, false);
        Self {
            stencil,
            inside,
            outside,
            new_inside: CoordSet::empty(),
            new_outside: CoordSet::empty(),
            new_explored: CoordSet::empty(),
            front: HashSet::new(),
        }
    }

    /// For a coord on the inside boundary
    /// Locate a stencil position that uses that inside cell
    pub fn find_stencil_position(
        &mut self,
        inside_coord: &Coord<GRID_DIMENSION>,
    ) {
        debug_assert!(self.inside.contains(inside_coord));
        let mut explored_boundary_coords = CoordSet::empty();
        // All the stencil positions we could try
        // For a candidate stencil position to be valid
        //  * No depdencies can touch the outside boundary
        //  * At least one dependency must on the inside
        //    - This one is implicit for find_stencil_position
        //    - Need to revisit for explore frontier
        for roi_offset in self.stencil.roi_offsets() {
            explored_boundary_coords.clear();

            // Candidate stencil center point
            let candidate_center = inside_coord + roi_offset;

            // Check the validity of that stencil position
            // By looking at every dependency
            let mut works = true;
            for rod_offset in self.stencil.offsets() {
                let dependency = candidate_center + rod_offset;

                // If a depdencency touches an outside coord
                // then this candidate center is not valid
                if self.outside.contains(&dependency) {
                    works = false;
                    break;
                }

                // If this candidate is valid,
                // then we want to note all of the new inside coord
                // we will be touching
                if let Some(false) = self.inside.get(&dependency) {
                    explored_boundary_coords.add(dependency);
                }
            }

            // We found a candidate
            // Update the state
            if works {
                self.new_inside.add(candidate_center);
                // Mark coords as explored
                for coord in explored_boundary_coords.coord_iter() {
                    debug_assert!(self.inside.contains(coord));
                    self.inside.add(*coord, true);
                }

                // Update our new center

                // Update frontier
                // One-ball around center,
                // but not positions on inside or outside or explored
                for offset in AABB::one_ball_iter() {
                    let frontier_candidate = candidate_center + offset;

                    if !self.new_explored.contains(&frontier_candidate)
                        && !self.new_inside.contains(&frontier_candidate)
                        && !self.new_outside.contains(&frontier_candidate)
                    {
                        self.front.insert(frontier_candidate);
                    }
                }

                break;
            }
            // If this candidate doesn't work then its outside
            else {
                self.new_outside.add(candidate_center);
            }
        }
    }

    pub fn explore_frontier_candidate(&mut self) {
        let frontier_candidate = *self.front.iter().next().unwrap();
        self.front.remove(&frontier_candidate);

        // Check all stencil deps
        // three cases,
        // - Touches outside
        //   - Add to new_outside
        // - no outside, touches inside
        //   - Add to new_inside,
        //   - Expand frontier
        // - Doesn't hit either, presume inside, add to explored
        let mut touches_inside = false;
        let mut explored_boundary_coords = CoordSet::empty();

        for offset in self.stencil.offsets() {
            let dependency = frontier_candidate + offset;

            if self.outside.contains(&dependency) {
                self.new_outside.add(frontier_candidate);
                return;
            }

            if let Some(v) = self.inside.get(&dependency) {
                touches_inside = true;
                if !v {
                    explored_boundary_coords.add(dependency)
                }
            }
        }

        if touches_inside {
            self.new_inside.add(frontier_candidate);
            for coord in explored_boundary_coords.coord_iter() {
                debug_assert!(self.inside.contains(coord));
                self.inside.add(*coord, true);
            }

            // Update frontier
            // One-ball around center,
            // but not positions on inside or outside or explored
            for offset in AABB::one_ball_iter() {
                let new_candidate = frontier_candidate + offset;

                if !self.new_explored.contains(&new_candidate)
                    && !self.new_inside.contains(&new_candidate)
                    && !self.new_outside.contains(&new_candidate)
                {
                    self.front.insert(frontier_candidate);
                }
            }
        } else {
            self.new_explored.add(frontier_candidate);
        }
    }

    pub fn find_candidate_inside(&self) -> Option<Coord<GRID_DIMENSION>> {
        for (coord, val) in self.inside.coord_iter() {
            if !val {
                return Some(*coord);
            }
        }

        None
    }

    pub fn step(&mut self) -> bool {
        if !self.front.is_empty() {
            self.explore_frontier_candidate();
            return true;
        }

        if let Some(inside_coord) = self.find_candidate_inside() {
            self.find_stencil_position(&inside_coord);
        }

        // Is there something in the frontier?
        // if so, try that
        // otherwise, grab find an unexplored coord
        // If none there then we're done

        false
    }
}
