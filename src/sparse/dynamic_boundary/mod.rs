mod borrow;
mod coord_set;
mod dilate;
mod find_boundary;
mod image_loader;
mod owned;
mod vtk;

pub use borrow::*;
pub use coord_set::*;
pub use dilate::*;
pub use find_boundary::*;
pub use image_loader::*;
pub use owned::*;
pub use vtk::*;

pub trait DynamicBoundary<const GRID_DIMENSION: usize> {
    fn inside(&self) -> &CoordSet<GRID_DIMENSION>;
    fn outside(&self) -> &CoordSet<GRID_DIMENSION>;
    fn to_owned(self) -> OwnedDynamicBoundary<GRID_DIMENSION>;
}
