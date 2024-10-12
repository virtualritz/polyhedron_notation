use crate::*;

impl Polyhedron {
    /// Adds edges from the center to each original vertex.
    ///
    /// # Arguments
    ///
    /// * `ratio` – The ratio of the new vertices to the original vertices.
    /// * `height` – The height of the new vertices.
    /// * `vertex_valence_mask` – Only vertices matching the given valences will
    ///   be affected.
    /// * `regular_faces_only` – Only regular faces will be affected.
    pub fn medial(
        &mut self,
        ratio: Option<Float>,
        height: Option<Float>,
        vertex_valence_mask: Option<&[usize]>,
        regular_faces_only: Option<bool>,
        change_name: bool,
    ) -> &mut Self {
        // TODO: Defaults

        self.ambo(ratio, false);
        self.truncate(height, vertex_valence_mask, regular_faces_only, false);
        self.dual(false);

        if change_name {
            let mut params = String::new();
            if let Some(ratio) = ratio {
                write!(&mut params, "{}", format_float(ratio)).unwrap();
            }
            if let Some(height) = height {
                write!(&mut params, ",{}", format_float(height)).unwrap();
            } else {
                write!(&mut params, ",").unwrap();
            }
            if let Some(vertex_valence_mask) = vertex_valence_mask {
                write!(
                    &mut params,
                    ",{}",
                    format_integer_slice(vertex_valence_mask)
                )
                .unwrap();
            } else {
                write!(&mut params, ",").unwrap();
            }
            if let Some(regular_faces_only) = regular_faces_only {
                if regular_faces_only {
                    params.push_str(",{t}");
                }
            } else {
                write!(&mut params, ",").unwrap();
            }
            params = params.trim_end_matches(',').to_string();
            self.name = format!("M{}{}", params, self.name);
        }

        self
    }
}
