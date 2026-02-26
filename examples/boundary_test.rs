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
    //let stencil = nhls::standard_stencils::simple_3pt_2d();
    let stencil = nhls::standard_stencils::offset_4pt_2d();

    let mut region_vtk_builder = CoordSetVTKBuilder2D::empty();
    let domain = region_from_image(&args.domain);
    let mut dyn_bound = find_region_boundaries(&domain, &stencil);
    region_vtk_builder.add_coord_set(dyn_bound.inside(), 0.0);
    region_vtk_builder.add_coord_set(dyn_bound.outside(), -1.0);

    let mut static_builder = CoordSetVTKBuilder2D::empty();
    let static_bounds = find_region_boundaries_static_rad(&domain, &stencil);
    static_builder.add_coord_set(&domain, 0.0);
    static_builder.add_coord_set(static_bounds.inside(), 1.0);
    static_builder.add_coord_set(static_bounds.outside(), -1.0);
    let static_path = args.output_dir.join("static_bound.vtu");
    static_builder.write(&static_path);

    let dynamic = find_dynamic_boundary(&domain, &stencil);
    let dynamic_path = args.output_dir.join("dynamic.vtu");
    write_dynamic_boundary(&dynamic, &dynamic_path);

    // modified
    let mod_in = static_bounds.inside().remove(dyn_bound.inside());
    let mod_out = static_bounds.outside().remove(dyn_bound.outside());
    let mut mod_builder = CoordSetVTKBuilder2D::empty();
    mod_builder.add_coord_set(&mod_in, 0.0);
    mod_builder.add_coord_set(&mod_out, -1.0);
    let mod_path = args.output_dir.join("mod_bound.vtu");
    mod_builder.write(&mod_path);

    let vtu_path = args.output_dir.join("dyn_bound.vtu");
    region_vtk_builder.write(&vtu_path);

    // create dilate loop
    let mut dilate_vtk_builder = CoordSetVTKBuilder2D::empty();
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

    // Set dilation
    let mut region = domain;
    let mut dilate_set_builder = CoordSetVTKBuilder2D::empty();
    let mut a_set_builder = CoordSetVTKBuilder2D::empty();
    let mut b_set_builder = CoordSetVTKBuilder2D::empty();
    dilate_set_builder.add_coord_set(&region, 0.0);
    let mut z = 1.0;
    while !region.is_empty() && z < 20.0 {
        println!("set iter: {}", z);
        let new_region = dilate_in_coord_set(&region, &stencil);
        let a = region.remove(&new_region);
        a_set_builder.add_coord_set(&a, z);
        let b = new_region.remove(&region);
        b_set_builder.add_coord_set(&b, z);
        region = new_region;
        dilate_set_builder.add_coord_set(&region, z);
        z += 1.0;
    }
    let dilate_set_vtu_path = args.output_dir.join("dilate_set.vtu");
    dilate_set_builder.write(&dilate_set_vtu_path);
    let a_vtu_path = args.output_dir.join("a_set.vtu");
    let b_vtu_path = args.output_dir.join("b_set.vtu");
    a_set_builder.write(&a_vtu_path);
    b_set_builder.write(&b_vtu_path);
}
