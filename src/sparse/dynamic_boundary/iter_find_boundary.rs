use std::path::PathBuf;

use crate::sparse::dynamic_boundary::*;
use crate::util::*;

// Phases,
// Iterate pointer
// get one element,
// head west?
// until boundary,

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

enum State {
    FindBoundary,
    TraverseBoundary,
}

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

pub struct Explorer {
    prior_pointer: Coord<2>,
    pointer: Coord<2>,
    state: State,
}

// So there's the evaluated region,
// Then there's the frontier we need to add to
// whenever we move the stencil

impl Explorer {
    fn new(start_coord: Coord<2>) -> Self {
        Self {
            prior_pointer: start_coord,
            pointer: start_coord,
            state: State::FindBoundary,
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

pub struct Driver {
    output_dir: PathBuf,
    explorer: Explorer,
    evaluator: Evaluator,
    step: usize,
}

impl Driver {
    pub fn new(domain: &CoordSet<2>, output_dir: PathBuf) -> Self {
        let start_coord = *domain.coord_iter().next().unwrap();
        let explorer = Explorer::new(start_coord);
        let evaluator = Evaluator::new(domain.clone());

        let result = Self {
            output_dir,
            explorer,
            evaluator,
            step: 0,
        };
        result.write();
        result
    }

    pub fn write(&self) {
        let expl_name = format!("explorer_{:04}.vtu", self.step);
        let expl_path = self.output_dir.join(expl_name);
        self.explorer.write_vtu(&expl_path);
        let eval_name = format!("evaluator_{:04}.vtu", self.step);
        let eval_path = self.output_dir.join(eval_name);
        self.evaluator.write_vtu(&eval_path);
    }

    pub fn step(&mut self) {
        self.explorer.step(&mut self.evaluator);
        self.step += 1;
        self.write();
    }
}
