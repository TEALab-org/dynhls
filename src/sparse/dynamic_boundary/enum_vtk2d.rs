use crate::sparse::dynamic_boundary::*;
use vtkio::model::*;

pub struct EnumVTKBuilder2D {
    point_data_name: String,
    points: Vec<f32>,
    point_data: Vec<u8>,
    connectivity: Vec<u64>,
    cell_types: Vec<CellType>,
    offsets: Vec<u64>,
    base: u64,
    offset: u64,
}

impl EnumVTKBuilder2D {
    pub fn empty() -> Self {
        Self {
            point_data_name: "point_data".to_string(),
            points: Vec::new(),
            point_data: Vec::new(),
            connectivity: Vec::new(),
            cell_types: Vec::new(),
            offsets: Vec::new(),
            base: 0,
            offset: 4,
        }
    }

    pub fn add_coord_set(&mut self, coord_set: &CoordSet<2>, z: f32, v: u8) {
        for coord in coord_set.coord_iter() {
            let x = coord[0] as f32;
            let y = coord[1] as f32;

            // A
            self.points.push(x);
            self.points.push(y);
            self.points.push(z);
            self.point_data.push(v);

            // B
            self.points.push(x + 1.0);
            self.points.push(y);
            self.points.push(z);
            self.point_data.push(v);

            // D
            self.points.push(x + 1.0);
            self.points.push(y + 1.0);
            self.points.push(z);
            self.point_data.push(v);

            // C
            self.points.push(x);
            self.points.push(y + 1.0);
            self.points.push(z);
            self.point_data.push(v);

            // Quad
            self.connectivity.push(self.base);
            self.connectivity.push(self.base + 1);
            self.connectivity.push(self.base + 2);
            self.connectivity.push(self.base + 3);
            self.cell_types.push(CellType::Quad);
            self.offsets.push(self.offset);
            self.offset += 4;
            self.base += 4;
        }
    }

    pub fn add_dynamic_boundary(
        &mut self,
        dynamic: &DynamicBoundary<2>,
        z: f32,
    ) {
        self.add_coord_set(dynamic.dilation_front.inside(), z, 0);
        self.add_coord_set(dynamic.dilation_front.outside(), z, 1);
        self.add_coord_set(dynamic.static_front.inside(), z, 2);
        self.add_coord_set(dynamic.static_front.outside(), z, 3);
    }

    pub fn set_point_data_name(&mut self, name: String) {
        self.point_data_name = name;
    }

    pub fn write<P: AsRef<std::path::Path>>(self, path: &P) {
        let model = Vtk {
            version: Version::XML { major: 1, minor: 0 },
            title: String::new(),
            byte_order: ByteOrder::LittleEndian,
            file_path: None,
            data: DataSet::inline(UnstructuredGridPiece {
                points: IOBuffer::F32(self.points),
                cells: Cells {
                    cell_verts: VertexNumbers::XML {
                        connectivity: self.connectivity,
                        offsets: self.offsets,
                    },
                    types: self.cell_types,
                },
                data: Attributes {
                    point: vec![Attribute::DataArray(DataArrayBase {
                        name: self.point_data_name,
                        elem: ElementType::Scalars {
                            num_comp: 1,
                            lookup_table: None,
                        },
                        data: IOBuffer::U8(self.point_data),
                    })],
                    cell: vec![],
                },
            }),
        };

        model.export(path).unwrap();
    }
}
