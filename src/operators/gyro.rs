use crate::*;

const DEFAULT_GYRO_RATIO: Float = 1. / 3.;
const DEFAULT_GYRO_HEIGHT: Float = 1. / 2.;

impl Polyhedron {
    /// Splits each edge and connects new edges at the split point to the face
    /// centroid. Existing positions are retained.
    /// ![Gyro](https://upload.wikimedia.org/wikipedia/commons/thumb/f/f6/Conway_gC.png/200px-Conway_gC.png)
    ///
    /// # Arguments
    ///
    /// * `ratio` – The ratio at which the adjacent edges get split.
    /// * `height` – An offset to add to the face centroid point along the face
    ///   normal.
    /// * `change_name` – Whether to change the name of the mesh.
    pub fn gyro(
        &mut self,
        ratio: Option<Float>,
        height: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_GYRO_RATIO,
        };
        let height = match height {
            Some(h) => h.clamp(0.0, 1.0),
            None => DEFAULT_GYRO_HEIGHT,
        };

        let edges = self.to_edges();
        let reversed_edges: Edges =
            edges.par_iter().map(|edge| [edge[1], edge[0]]).collect();

        // Retain original positions, add face centroids and directed
        // edge positions each N-face becomes N pentagons.
        let new_positions: Vec<(&FaceSlice, Point)> = self
            .face_index
            .par_iter()
            .map(|face| {
                let fp = index_as_positions(face, &self.positions);
                (
                    face.as_slice(),
                    centroid_ref(&fp) + average_normal_ref(&fp) * height,
                )
            })
            .chain(edges.par_iter().enumerate().flat_map(|edge| {
                let edge_positions =
                    index_as_positions(edge.1, &self.positions);
                vec![
                    (
                        &edge.1[..],
                        *edge_positions[0]
                            + ratio * (*edge_positions[1] - *edge_positions[0]),
                    ),
                    (
                        &reversed_edges[edge.0][..],
                        *edge_positions[1]
                            + ratio * (*edge_positions[0] - *edge_positions[1]),
                    ),
                ]
            }))
            .collect();

        let new_ids = vertex_ids_ref_ref(
            &new_positions,
            self.positions_len() as VertexKey,
        );

        self.positions.extend(vertex_values_as_ref(&new_positions));

        self.face_index = self
            .face_index
            .par_iter()
            .flat_map(|face| {
                face.iter()
                    .cycle()
                    .skip(face.len() - 1)
                    .tuple_windows::<(_, _, _)>()
                    .take(face.len())
                    .map(|v| {
                        let a = *v.1;
                        let b = *v.2;
                        let z = *v.0;
                        let eab = vertex(&[a, b], &new_ids).unwrap();
                        let eza = vertex(&[z, a], &new_ids).unwrap();
                        let eaz = vertex(&[a, z], &new_ids).unwrap();
                        let centroid = vertex(face, &new_ids).unwrap();
                        vec![a, eab, centroid, eza, eaz]
                    })
                    .collect::<Faces>()
            })
            .collect();

        self.dual(false);
        self.dual(false);

        if change_name {
            let params = match ratio != DEFAULT_GYRO_RATIO
                || height != DEFAULT_GYRO_HEIGHT
            {
                true => {
                    format!("{},{}", format_float(ratio), format_float(height))
                }
                false => "".to_string(),
            };
            self.name = format!("g{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn g(&mut self) -> &mut Self {
        self.gyro(Some(DEFAULT_GYRO_RATIO), Some(DEFAULT_GYRO_HEIGHT), true)
    }
}
