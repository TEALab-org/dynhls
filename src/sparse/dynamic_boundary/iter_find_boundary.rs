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
    DecY
}

pub struct Explorer {
    prior_pointer: Coord<2>,
    pointer: Coord<2>,
    state: State,
}

type Neighborhood = nalgebra::SMatrix<bool, 3, 3>;

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
/*
    fn evaluate_neighborhood(&self, eval: &mut Evaluator) -> Neighborhood {
       Neighborhood::bi() 
    }
*/

    fn step(&mut self, eval: &mut Evaluator) {
        //0b000 000 000
        // State machine
        // 
        // Eval 3x3 AABB and pattern match?
        let neighborhood = 0;
        let (add, s) = match neighborhood {
            // -0-
            // 000
            // -0-
            0b00000 => panic!("Entirely Outside"), 
            // -0-
            // 000
            // -1-
            0b00001 => (false, State::IncY),
            // -0-
            // 001
            // -0-
            0b00010 => (false, State::IncX),
            // -0-
            // 001
            // -1-
            0b00011 => panic!("where go?"),
            // -0-
            // 010
            // -0-
            0b00100 => (true, State::IncY),
            // -0-
            // 010
            // -1-
            0b00101 => (true, State::IncX),
            // -0-
            // 011
            // -0-
            0b00110 => (true, State::IncX),
            // -0-
            // 011
            // -1-
            0b00111 => (false, State::DecX),
            // -0-
            // 100
            // -0-
            0b01000 => (false, State::IncY),
            // -0-
            // 100
            // -1-
            0b01001 => (false, State::IncX),
            // -0-
            // 101
            // -0-
            0b01010 => (false, State::IncX),
            // -0-
            // 101
            // -1-
            0b01011 => (true, State::DecX),
            // -0-
            // 110
            // -0-
            0b01100 => (true, State::IncY),
            // -0-
            // 110
            // -1-
            0b01101 => (true, State::IncX),
            // -0-
            // 111
            // -0-
            0b01110 => (true, State::IncX),
            // -0-
            // 111
            // -1-
            0b01111 => (false, State::DecY),
            // -1-
            // 000
            // -0-
            0b10000 => (false, State::DecY),
            // -1-
            // 000
            // -1-
            0b10001 => (false, State::IncX),
            // -1-
            // 001
            // -0-
            0b10010 => (false, State::DecY),
            // -1-
            // 001
            // -1-
            0b10011 => (false, State::DecY),
            // -1-
            // 010
            // -0-
            0b10100 => (true, State::DecX),
            // -1-
            // 010
            // -1-
            0b10101 => (true, State::DecY),
            // -1-
            // 011
            // -0-
            0b10110 => (true, State::DecY),
            // -1-
            // 011
            // -1-
            0b10111 => (true, State::DecY),
            // -1-
            // 100
            // -0-
            0b11000 => (false, State::DecX),
            // -1-
            // 100
            // -1-
            0b11001 => (false, State::IncY),
            // -1-
            // 101
            // -0-
            0b11010 => (false, State::DecX),
            // -1-
            // 101
            // -1-
            0b11011 => (false, State::DecX),
            // -1-
            // 110
            // -0-
            0b11100 => (true, State::DecX),
            // -1-
            // 110
            // -1-
            0b11101 => (true, State::IncY),
            // -1-
            // 111
            // -0-
            0b11110 => (true, State::DecX),
            // -1-
            // 111
            // -1-
            0b11111 => (false, State::DecX),
            default => panic!("Wut"),
        }

        // I think we're gonna need to add something about 

    }

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
        // TODO, we should use ROI offset to get a start point I think?
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
