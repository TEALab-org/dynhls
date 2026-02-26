use crate::sparse::dynamic_boundary::*;

pub fn write_dynamic_boundary<P: AsRef<std::path::Path>>(dynamic: &DynamicBoundary<2>, path: &P) {
    let mut builder = CoordSetVTKBuilder2D::empty();
    builder.add_coord_set_value(dynamic.dilation_front.inside(), 0.0, 0.0);
    builder.add_coord_set_value(dynamic.dilation_front.outside(), 0.0, 1.0);
    builder.add_coord_set_value(dynamic.static_front.inside(), 0.0, 2.0);
    builder.add_coord_set_value(dynamic.static_front.outside(), 0.0, 3.0);
    builder.write(path);
}


