use crate::domain::*;
use crate::util::*;
use image::ImageReader;

pub fn domain_from_image<P: AsRef<std::path::Path>>(path: &P) -> OwnedDomain<2> {
    let image = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();

    // Into raw TODO

    let width = image.width() as i32;
    let height = image.height() as i32;
    let bounds = AABB::new(matrix![0, width - 1; 0, height - 1]);
    OwnedDomain::new(bounds)
}
