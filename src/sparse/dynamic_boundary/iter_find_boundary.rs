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
