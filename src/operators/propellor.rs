use crate::*;

const DEFAULT_PROPELLOR_RATIO: Float = 1. / 3.;

impl Polyhedron {
    /// Splits each edge into three parts and creates edges on each face
    /// connecting the new vertices.
    ///
    /// # Arguments
    ///
    /// * `ratio` – The ratio of the edge splits.
    pub fn propellor(
        &mut self,
        ratio: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_PROPELLOR_RATIO,
        };

        let edges = self.to_edges();
        let reversed_edges: Edges =
            edges.par_iter().map(|edge| [edge[1], edge[0]]).collect();

        let new_positions = edges
            .iter()
            .zip(reversed_edges.iter())
            .flat_map(|(edge, reversed_edge)| {
                let edge_positions = index_as_positions(edge, &self.positions);
                vec![
                    (
                        edge,
                        *edge_positions[0]
                            + ratio * (*edge_positions[1] - *edge_positions[0]),
                    ),
                    (
                        reversed_edge,
                        *edge_positions[1]
                            + ratio * (*edge_positions[0] - *edge_positions[1]),
                    ),
                ]
            })
            .collect::<Vec<_>>();

        let new_ids = vertex_ids_edge_ref_ref(
            &new_positions,
            self.positions_len() as VertexKey,
        );

        self.face_index = self
            .face_index
            .par_iter()
            .map(|face| {
                // Rotated face.
                face.iter()
                    .circular_tuple_windows::<(_, _)>()
                    .map(|f| vertex_edge(&[*f.0, *f.1], &new_ids).unwrap())
                    .collect()
            })
            .chain(self.face_index.par_iter().flat_map(|face| {
                (0..face.len())
                    .map(|j| {
                        let a = face[j];
                        let b = face[(j + 1) % face.len()];
                        let z = face[(j + face.len() - 1) % face.len()];
                        let edge_ab = vertex_edge(&[a, b], &new_ids).unwrap();
                        let edge_ba = vertex_edge(&[b, a], &new_ids).unwrap();
                        let edge_za = vertex_edge(&[z, a], &new_ids).unwrap();

                        vec![a, edge_ba, edge_ab, edge_za]
                    })
                    .collect::<Faces>()
            }))
            .collect::<Faces>();

        self.positions.extend(vertex_values_as_ref(&new_positions));

        if change_name {
            let params = match ratio != DEFAULT_PROPELLOR_RATIO {
                true => format_float(ratio),
                false => "".to_string(),
            };
            self.name = format!("p{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn p(&mut self) -> &mut Self {
        self.propellor(Some(DEFAULT_PROPELLOR_RATIO), true)
    }
}
