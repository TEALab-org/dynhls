use crate::sparse::dynamic_boundary::CoordSet;
use crate::util::*;

use std::collections::HashMap;

#[derive(Clone)]
pub struct CoordMap<const GRID_DIMENSION: usize, DataType: Clone> {
    cells: HashMap<Coord<GRID_DIMENSION>, DataType>,
    aabb: AABB<GRID_DIMENSION>,
}

impl<const GRID_DIMENSION: usize, DataType: Clone>
    CoordMap<GRID_DIMENSION, DataType>
{
    pub fn empty() -> Self {
        Self {
            cells: HashMap::new(),
            aabb: AABB::empty(),
        }
    }

    pub fn from_coord_set(
        set: &CoordSet<GRID_DIMENSION>,
        value: DataType,
    ) -> Self {
        let mut result = Self::empty();
        result
            .cells
            .extend(set.cells.iter().map(|c| (c.clone(), value.clone())));
        result
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn add(&mut self, coord: Coord<GRID_DIMENSION>, data: DataType) {
        self.aabb.add_coord(&coord);
        self.cells.insert(coord, data);
    }

    pub fn contains(&self, coord: &Coord<GRID_DIMENSION>) -> bool {
        self.cells.contains_key(coord)
    }

    pub fn get(&self, coord: &Coord<GRID_DIMENSION>) -> Option<&DataType> {
        self.cells.get(coord)
    }

    pub fn coord_iter(
        &self,
    ) -> impl Iterator<Item = (&Coord<GRID_DIMENSION>, &DataType)> {
        self.cells.iter()
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.aabb = AABB::empty();
    }
}
