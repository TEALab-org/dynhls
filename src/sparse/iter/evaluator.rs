use crate::sparse::dynamic_boundary::*;
use crate::util::*;

#[derive(Copy, Clone)]
pub enum EvalState {
    In,
    Out,
}

impl ToVTKU8 for EvalState {
    fn to_vtk_u8(&self) -> u8 {
        match self {
            EvalState::In => 1,
            EvalState::Out => 0,
        }
    }
}

pub struct Evaluator {
    region: CoordSet<2>,
    evaled: CoordMap<2, EvalState>,
}

impl Evaluator {
    pub fn new(region: CoordSet<2>) -> Self {
        let evaled = CoordMap::empty();
        Self { region, evaled }
    }

    pub fn eval(&mut self, coord: &Coord<2>) -> bool {
        if self.region.contains(coord) {
            self.evaled.add(*coord, EvalState::Out);
            true
        } else {
            self.evaled.add(*coord, EvalState::Out);
            false
        }
    }

    pub fn write_vtu<P: AsRef<std::path::Path>>(&self, path: &P) {
        write_coord_map(&self.evaled, path);
    }
}
