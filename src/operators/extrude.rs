use crate::*;

const DEFAULT_EXTRUDE_HEIGHT: Float = 1. / 10.;

impl Polyhedron {
    /// Extrudes faces by `height` and shrinks the extruded faces by `distance`
    /// from the original edges.
    ///
    /// # Arguments
    ///
    /// * `height` – The distance to extrude the faces. Default value is `0.1`.
    /// * `offset` – The distance to inset the extruded faces. Default value is
    ///   `0.0`.
    /// * `face_arity_mask` – Only faces matching the given arities will be
    ///   affected.
    pub fn extrude(
        &mut self,
        height: Option<Float>,
        offset: Option<Float>,
        face_arity: Option<&[usize]>,
        change_name: bool,
    ) -> &mut Self {
        let new_positions = self
            .face_index
            .par_iter()
            .filter(|face| face_arity_matches(face, face_arity))
            .flat_map(|face| {
                let face_positions = index_as_positions(face, &self.positions);
                let centroid = centroid_ref(&face_positions);
                face.iter()
                    .zip(&face_positions)
                    .map(|face_vertex_point| {
                        (
                            extend![..face, *face_vertex_point.0],
                            **face_vertex_point.1
                                + offset.unwrap_or(0.0)
                                    * (centroid - **face_vertex_point.1)
                                + average_normal_ref(&face_positions)
                                    * height.unwrap_or(DEFAULT_EXTRUDE_HEIGHT),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let new_ids =
            vertex_ids_ref(&new_positions, self.positions_len() as VertexKey);

        self.face_index = self
            .face_index
            .par_iter()
            .flat_map(|face| {
                if face_arity_matches(face, face_arity) {
                    face.iter()
                        .enumerate()
                        .flat_map(|index_vertex| {
                            let a = *index_vertex.1;
                            let inset_a =
                                vertex(&extend![..face, a], &new_ids).unwrap();
                            let b = face[(index_vertex.0 + 1) % face.len()];
                            let inset_b =
                                vertex(&extend![..face, b], &new_ids).unwrap();
                            if height.unwrap_or(0.3).is_sign_positive() {
                                vec![vec![a, b, inset_b, inset_a]]
                            } else {
                                vec![vec![inset_a, inset_b, b, a]]
                            }
                        })
                        .chain(vec![face
                            .iter()
                            .map(|v| {
                                vertex(&extend![..face, *v], &new_ids).unwrap()
                            })
                            .collect::<Vec<_>>()])
                        .collect::<Vec<_>>()
                } else {
                    vec![face.clone()]
                }
            })
            .collect();

        self.positions.extend(vertex_values_as_ref(&new_positions));

        if change_name {
            let mut params = String::new();
            if let Some(height) = height {
                write!(&mut params, "{}", format_float(height)).unwrap();
            }
            if let Some(offset) = offset {
                write!(&mut params, ",{}", format_float(offset)).unwrap();
            } else {
                write!(&mut params, ",").unwrap();
            }
            if let Some(face_arity) = face_arity {
                write!(&mut params, ",{}", format_integer_slice(face_arity))
                    .unwrap();
            } else {
                write!(&mut params, ",").unwrap();
            }
            params = params.trim_end_matches(',').to_string();
            self.name = format!("x{}{}", params, self.name);
        }

        self
    }
}
