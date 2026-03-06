use crate::sparse::dynamic_boundary::*;
use crate::util::*;

// Phases,
// Iterate pointer
// get one element,
// head west?
// until boundary,

#[derive(Copy, Clone)]
enum PointerType {
    Pointer,
    Prior,
}

impl ToVTKU8 for PointerType {
    fn to_vtk_u8(&self) -> u8 {
        match self {
            PointerType::Pointer => 3,
            PointerType::Prior => 4,
        }
    }
}

enum State {
    First,
    IncX,
    IncY,
    DecX,
    DecY,
}

pub struct Explorer {
    prior_pointer: Coord<2>,
    pointer: Coord<2>,
    state: State,
}

// So there's the evaluated region,
// Then there's the frontier we need to add to
// whenever we move the stencil

// For now lets trace the boundary with 1-1 stencil
impl Explorer {
    fn new(start_coord: Coord<2>) -> Self {
        Self {
            prior_pointer: start_coord,
            pointer: start_coord,
            state: State::First,
        }
    }
    fn step(&mut self, eval: &mut Evaluator) {}

    fn write_vtu<P: AsRef<std::path::Path>>(&self, path: &P) {
        let mut pointer_map = CoordMap::empty();
        pointer_map.add(self.pointer, PointerType::Pointer);
        pointer_map.add(self.prior_pointer, PointerType::Prior);
        write_coord_map(&pointer_map, path);
    }
}
