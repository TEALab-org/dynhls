use clap::Parser;
use nhls::sparse::dynamic_boundary::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to place all vtu files
    #[arg(short, long)]
    pub output_dir: PathBuf,

    /// Input Domain
    #[arg(short, long)]
    pub domain: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("domain: {:?}", args.domain);
    println!("output_dir: {:?}", args.output_dir);
    std::fs::create_dir_all(&args.output_dir).unwrap();

    //let stencil = nhls::standard_stencils::heat_2d(1.0, 1.0, 1.0, 0.2, 0.2);
    let stencil = nhls::standard_stencils::simple_3pt_2d();

    let mut region_vtk_builder = CoordSetVTKBuilder::empty();

    let domain = region_from_image(&args.domain);
    region_vtk_builder.add_coord_set(&domain, 0.0);
    let mut dyn_bound = find_region_boundaries(&domain, &stencil);
    region_vtk_builder.add_coord_set(dyn_bound.inside(), 1.0);
    region_vtk_builder.add_coord_set(dyn_bound.outside(), -1.0);

    let vtu_path = args.output_dir.join("region.vtu");
    region_vtk_builder.write(&vtu_path);

    // create dilate loop
    let mut dilate_vtk_builder = CoordSetVTKBuilder::empty();
    dilate_vtk_builder.add_coord_set(dyn_bound.inside(), 1.0);
    let mut z = 2.0;
    while !dyn_bound.inside().is_empty() && z < 40.0 {
        println!("iter: {}", z);
        dyn_bound = dilate_in(&dyn_bound, &stencil);
        dilate_vtk_builder.add_coord_set(dyn_bound.inside(), z);
        z += 1.0;
    }

    let dilate_vtu_path = args.output_dir.join("dilate.vtu");
    dilate_vtk_builder.write(&dilate_vtu_path);
}
