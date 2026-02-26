use crate::sparse::dynamic_boundary::*;
use crate::util::*;

pub struct DynamicBoundary<const GRID_DIMENSION: usize> {
    pub dilation_front: OwnedBoundaryPiece<GRID_DIMENSION>,
    pub static_front: OwnedBoundaryPiece<GRID_DIMENSION>,
}

impl<const GRID_DIMENSION: usize> DynamicBoundary<GRID_DIMENSION> {
    pub fn new(
        dilation_front: OwnedBoundaryPiece<GRID_DIMENSION>,
        static_front: OwnedBoundaryPiece<GRID_DIMENSION>,
    ) -> Self {
        let result = Self {
            dilation_front,
            static_front,
        };
        debug_assert!(result.check_invariant());
        result
    }

    pub fn check_invariant(&self) -> bool {
        let mut check_map = std::collections::HashMap::new();
        let add_set = |set: &CoordSet<GRID_DIMENSION>,
                       map: &mut std::collections::HashMap<
            Coord<GRID_DIMENSION>,
            usize,
        >| {
            for coord in set.coord_iter() {
                if map.contains_key(&coord) {
                    let c = map.get(coord).unwrap();
                    map.insert(*coord, c + 1);
                } else {
                    map.insert(*coord, 1);
                }
            }
        };
        add_set(self.dilation_front.inside(), &mut check_map);
        add_set(self.dilation_front.outside(), &mut check_map);
        add_set(self.static_front.inside(), &mut check_map);
        add_set(self.static_front.outside(), &mut check_map);
        let mut result = true;
        for (k, v) in check_map.iter() {
            if *v != 1 {
                result = false;
                println!("DYNAMIC BOUND INVAR ERROR: {} -> {}", k, v);
            }
        }
        result
    }
}
