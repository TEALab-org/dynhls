use clap::Parser;
use nhls::domain::*;
use nhls::image::*;
use nhls::sparse::dynamic_boundary::*;
use nhls::sparse::iter::initial_oneball_boundary;
use nhls::stencil::*;
use nhls::util::*;
use std::path::PathBuf;
use std::io::prelude::*;
use std::io::BufWriter;

use noise::{
    core::simplex::{simplex_2d, simplex_3d, simplex_4d},
    permutationtable::PermutationTable,
    utils::*,
    MultiFractal, Fbm, Perlin, Worley, Simplex,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to place result
    #[arg(short, long)]
    pub output: PathBuf,

    #[arg(short, long)]
    pub n: usize,

    /// Try 5.0, must be positive
    #[arg(short, long)]
    pub scale: f64,

    #[arg(long)]
    pub octaves: usize,

    #[arg(short, long)]
    pub freq: f64,
}

fn find_region(domain: &OwnedDomain<2>) -> (CoordSet<2>, OwnedDomain<2>) {
    let mut a = CoordSet::empty();
    let mut mask = OwnedDomain::new(*domain.aabb());

    for coord in domain.aabb().coord_iter() {
        if domain.view(&coord) > 0.0 {
            a.add(coord);
            mask.set_coord(&coord, 1.0);
        }
    }
    (a, mask)
}

fn test_params(n: usize, scale: f64, octaves: usize, write_image: Option<&PathBuf>) -> f64 {
    let chunk_size = 10000;
    let noise_func = Fbm::<Simplex>::default().set_octaves(octaves);
    let noise_map = 
        PlaneMapBuilder::new(noise_func)
            .set_size(n, n)
            .set_x_bounds(-scale, scale)
            .set_y_bounds(-scale, scale)
            .build();

    let n_inclusive = n as i32 - 1;
    let grid_bounds = AABB::new(matrix![0, n_inclusive; 0, n_inclusive]);
    let mut domain = OwnedDomain::new(grid_bounds);

    domain.par_set_values(
        |world_coord: Coord<2>| {
            debug_assert!(world_coord[0] >= 0);
            debug_assert!(world_coord[1] >= 0);

            let x = world_coord[0] as usize;
            let y = world_coord[1] as usize;
            noise_map.get_value(x, y)
        },
        chunk_size,
    );

    let (region, mask) = find_region(&domain);

    if let Some(output_prefix) = write_image {
        let (min, max) = domain
            .buffer()
            .iter()
            .fold((f64::MAX, f64::MIN), |(acc_min, acc_max), x| {
                (acc_min.min(*x), acc_max.max(*x))
            });
        println!("MIN: {}, MAX: {}", min, max);
        image2d_scaled(&domain, &output_prefix.join("ic_2.png"), min, max);
        image2d(&mask, &output_prefix.join("mask_2.png"));
    }

    let ob = initial_oneball_boundary(&region);

    let result = ob.inside.len() as f64 / (n * n) as f64;
    
    //println!("s: {}, |R|: {}, |OB|: {}, r: {}", scale, region.len(), ob.inside.len(), result); 

    result
}

fn main() {
//    let chunk_size = 10000;
//    let args = Args::parse();
//    println!("output: {:?}", args.output);
//    debug_assert!(args.scale > 0.0);

    //let mut f = std::fs::File::create("target/noise/results.json").unwrap();
    //let mut output = BufWriter::new(f); 
    let n = 1024;
    let octaves = 6;
    let s_min = 0.2;
    let s_max = 25.0;
    let s_delta = 0.2;

    //let mut scales = Vec::new();
    //let mut rs = Vec::new();

    let mut scale = s_min;
    while scale <= s_max {
        let r = test_params(n, scale, octaves, None);
        println!("{}, {}", scale, r);
        scale += s_delta;
    }
}
