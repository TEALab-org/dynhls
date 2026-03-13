use clap::Parser;
use nhls::sparse::dynamic_boundary::*;
use nhls::sparse::iter::initial_oneball_boundary;
use nhls::sparse::iter::*;
use nhls::stencil::*;
use nhls::util::*;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum StencilFig {
    Grid,
    Roi,
    Rod,
    Present,
}

impl ToVTKU8 for StencilFig {
    fn to_vtk_u8(&self) -> u8 {
        match self {
            StencilFig::Grid => 0,
            StencilFig::Roi => 1,
            StencilFig::Rod => 2,
            StencilFig::Present => 3,
        }
    }
}

fn make_stencil_fig<
    const NEIGHBORHOOD_SIZE: usize,
    StencilType: TVStencil<2, NEIGHBORHOOD_SIZE>,
>(
    name: &str,
    stencil: &StencilType,
    output_dir: &PathBuf,
) {
    let r = stencil.radius() + 1;

    let mut layer_off = CoordMap::empty();
    let mut layer_rod = CoordMap::empty();
    let mut layer_present = CoordMap::empty();
    let mut layer_roi = CoordMap::empty();

    for x in -r..=r {
        for y in -r..=r {
            layer_off.add(vector![x, y], StencilFig::Grid);
        }
    }

    for offset in stencil.roi_offsets() {
        layer_roi.add(offset, StencilFig::Roi);
    }

    for offset in stencil.offsets() {
        layer_rod.add(*offset, StencilFig::Rod);
    }

    layer_present.add(Coord::zero(), StencilFig::Present);

    let mut builder = CoordMapVTKBuilder2D::empty();
    builder.add_coord_map(&layer_off, 0.0);
    builder.add_coord_map(&layer_off, 1.0);
    builder.add_coord_map(&layer_off, 2.0);
    builder.add_coord_map(&layer_rod, 0.0);
    builder.add_coord_map(&layer_present, 1.0);
    builder.add_coord_map(&layer_roi, 2.0);
    builder.set_point_data_name("stencil_fig".to_string());

    let vtu_path = output_dir.join(format!("s_{}.vtu", name));
    builder.write(&vtu_path);
}

fn write_explorer<'a, const N: usize, StencilType: TVStencil<2, N>>(
    output_dir: &PathBuf,
    step: usize,
    explorer: &Explorer<'a, 2, N, StencilType>,
) {
    let inside_path = output_dir.join(format!("new_inside_{:04}.vtu", step));
    let outside_path = output_dir.join(format!("new_outside_{:04}.vtu", step));
    let explored_path =
        output_dir.join(format!("new_explored_{:04}.vtu", step));
    let front_path = output_dir.join(format!("front_{:04}.vtu", step));

    write_coord_set(&explorer.new_inside, 1.0, &inside_path);
    write_coord_set(&explorer.new_outside, 1.0, &outside_path);
    write_coord_set(&explorer.new_explored, 1.0, &explored_path);
    write_coord_set(&explorer.front, 1.0, &front_path);
}

fn test_explorer<
    const GRID_DIMENSION: usize,
    const NEIGHBORHOOD_SIZE: usize,
    StencilType: TVStencil<GRID_DIMENSION, NEIGHBORHOOD_SIZE>,
>(
    domain: &CoordSet<GRID_DIMENSION>,
    stencil: &StencilType,
) {
    let init_boundary = initial_oneball_boundary(&domain);
    let fill_init_boundary = fill_boundary(&init_boundary);
    assert_eq!(&fill_init_boundary, domain);
    let explorer = Explorer::new(
        stencil,
        init_boundary.inside.clone(),
        init_boundary.outside.clone(),
    );
    let explorer_result = explorer.result();
    let fill_result = fill_boundary(&explorer_result);
    let expected = dilate_in_coord_set(domain, stencil);
    assert_eq!(fill_result, expected);
}

fn main() {
    let args = Args::parse();
    println!("domain: {:?}", args.domain);
    println!("output_dir: {:?}", args.output_dir);
    std::fs::create_dir_all(&args.output_dir).unwrap();

    //let stencil = nhls::standard_stencils::heat_2d(1.0, 1.0, 1.0, 0.2, 0.2);
    //let stencil = nhls::standard_stencils::simple_3pt_2d();
    //let stencil = nhls::standard_stencils::offset_4pt_2d();
    //let stencil = nhls::standard_stencils::vert_3pt_2d();

    let domain = region_from_image(&args.domain);
    test_explorer(&domain, &nhls::standard_stencils::heat_2d(1.0, 1.0, 1.0, 0.2, 0.2));
    test_explorer(&domain, &nhls::standard_stencils::simple_3pt_2d());
    test_explorer(&domain, &nhls::standard_stencils::vert_3pt_2d());
    test_explorer(&domain, &nhls::standard_stencils::offset_4pt_2d());


/*
 *
    println!("*** BEGIN TEST INIT / FILL ***");
    let init_boundary = initial_oneball_boundary(&domain);
    let fill_init_boundary = fill_boundary(&init_boundary);
    assert_eq!(fill_init_boundary, domain);

    println!("*** BEGIN EXPLORER SINGLE STEP ***");
    let mut explorer = Explorer::new(
        &stencil,
        init_boundary.inside.clone(),
        init_boundary.outside.clone(),
    );

    println!("*** BEGIN FINAL FILL ***");
    let explorer_result = explorer.result();

    write_coord_set(
        &explorer_result.inside,
        0.0,
        &args.output_dir.join("expl_inside.vtu"),
    );

    write_coord_set(
        &explorer_result.outside,
        0.0,
        &args.output_dir.join("expl_outside.vtu"),
    );

    let mut builder = CoordSetVTKBuilder2D::empty();
    let mut result = CoordSet::empty();
    let mut new_points = explorer_result.inside.clone();
    let mut next_points = CoordSet::empty();
    let mut step = 0.0;
    while !new_points.is_empty() && step < 20.1 {
        builder.add_coord_set_value(&new_points, 0.0, step); 
        step += 1.0;
        for coord in new_points.coord_iter() {
            result.add(*coord);
            for offset in AABB::one_ball_iter() {
                let n_coord = coord + offset;
                if !explorer_result.outside.contains(&n_coord) 
                && !result.contains(&n_coord) {
                    next_points.add(n_coord);
                }
            }
        }
        
        std::mem::swap(&mut new_points, &mut next_points);
        next_points.clear();

        println!("result: {}", result.len());
    }
    builder.write(&args.output_dir.join("explr_fill.vtu"));
*/

    /*
        make_stencil_fig("stenc", &stencil, &args.output_dir);

        let init_boundary = initial_oneball_boundary(&domain);
        let inside_path = args.output_dir.join("init_inside.vtu");
        let outside_path = args.output_dir.join("init_outside.vtu");
        write_coord_set(&init_boundary.inside, 0.0, &inside_path);
        write_coord_set(&init_boundary.outside, 0.0, &outside_path);

        let mut explorer =
            Explorer::new(&stencil, init_boundary.inside, init_boundary.outside);

        let mut step = 1;
        while explorer.step() && step < 1000 {
            println!("step: {}", step);
            explorer.report();
            write_explorer(&args.output_dir, step, &explorer);
            step += 1;
        }
    */
}
