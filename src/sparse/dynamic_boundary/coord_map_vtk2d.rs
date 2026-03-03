use crate::sparse::dynamic_boundary::*;
use vtkio::model::*;

pub trait ToVTKU8 {
    fn to_vtk_u8(&self) -> u8;
}

pub fn write_coord_map<P: AsRef<std::path::Path>, DataType: Clone + ToVTKU8>(
    coord_map: &CoordMap<2, DataType>,
    path: &P,
) {
    let mut builder = CoordMapVTKBuilder2D::empty();
    builder.add_coord_map(coord_map, 0.0);
    builder.write(path)
}

pub struct CoordMapVTKBuilder2D {
    points: Vec<f32>,
    point_data: Vec<u8>,
    connectivity: Vec<u64>,
    cell_types: Vec<CellType>,
    offsets: Vec<u64>,
    base: u64,
    offset: u64,
}

impl CoordMapVTKBuilder2D {
    pub fn empty() -> Self {
        Self {
            points: Vec::new(),
            point_data: Vec::new(),
            connectivity: Vec::new(),
            cell_types: Vec::new(),
            offsets: Vec::new(),
            base: 0,
            offset: 4,
        }
    }

    pub fn add_coord_map<DataType: Clone + ToVTKU8>(
        &mut self,
        coord_map: &CoordMap<2, DataType>,
        z: f32,
    ) {
        for (coord, data) in coord_map.coord_iter() {
            let x = coord[0] as f32;
            let y = coord[1] as f32;
            let v = data.to_vtk_u8();

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
                        name: String::from("point_data"),
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
