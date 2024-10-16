use crate::*;

const DEFAULT_KIS_HEIGHT: Float = 1. / 10.;

impl Polyhedron {
    /// Creates a [kleetrope](https://en.wikipedia.org/wiki/Kleetope) from the
    /// input. Splits each face into triangles, one for each edge, which
    /// extend to the face centroid. Existing positions are retained.
    ///
    /// # Arguments
    ///
    /// * `height` - An offset to add to the face centroid point along the face
    ///   normal.
    /// * `face_arity_mask` - Only faces matching the given arities will be
    ///   affected.
    /// * `face_index_mask` - Only faces matching the given indices will be
    ///   affected.
    /// * `regular_faces_only` - Only faces whose edges are 90% the same length,
    ///   within the same face, are affected.
    pub fn kis(
        &mut self,
        height: Option<Float>,
        face_arity_mask: Option<&[usize]>,
        face_index_mask: Option<&[FaceKey]>,
        regular_faces_only: Option<bool>,
        change_name: bool,
    ) -> &mut Self {
        let height = match height {
            Some(h) => h.clamp(0.0, 1.0),
            None => DEFAULT_KIS_HEIGHT,
        };

        let new_positions: Vec<(&FaceSlice, Point)> = self
            .face_index
            .par_iter()
            .enumerate()
            .filter_map(|(index, face)| {
                if is_face_selected(
                    face,
                    index,
                    &self.positions,
                    face_arity_mask,
                    face_index_mask,
                    regular_faces_only,
                ) {
                    let face_positions =
                        index_as_positions(face, &self.positions);
                    Some((
                        face.as_slice(),
                        centroid_ref(&face_positions)
                            + average_normal_ref(&face_positions) * height,
                    ))
                } else {
                    None
                }
            })
            .collect();

        let new_ids = vertex_ids_ref_ref(
            &new_positions,
            self.positions.len() as VertexKey,
        );

        self.positions.extend(vertex_values_as_ref(&new_positions));

        self.face_index = self
            .face_index
            .par_iter()
            .flat_map(|face: &Face| match vertex(face, &new_ids) {
                Some(centroid) => face
                    .iter()
                    .cycle()
                    .tuple_windows::<(&VertexKey, _)>()
                    .take(face.len())
                    .map(|v| vec![*v.0, *v.1, centroid as VertexKey])
                    .collect(),
                None => vec![face.clone()],
            })
            .collect();

        if change_name {
            let mut params = match height != DEFAULT_KIS_HEIGHT {
                true => format!("{}", format_float(height)),
                false => "".to_string(),
            };
            if let Some(face_arity_mask) = face_arity_mask {
                params = format!(
                    "{params},{}",
                    format_integer_slice(face_arity_mask)
                );
            }
            if let Some(face_index_mask) = face_index_mask {
                params = format!(
                    "{params},{}",
                    format_integer_slice(face_index_mask)
                );
            }
            if let Some(regular_faces_only) = regular_faces_only {
                if regular_faces_only {
                    params.push_str(",{t}");
                }
            }
            self.name = format!("k{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn k(&mut self) -> &mut Self {
        self.kis(Some(DEFAULT_KIS_HEIGHT), None, None, Some(false), true)
    }
}
