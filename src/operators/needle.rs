use crate::*;

impl Polyhedron {
    /// Like [`kis`](Polyhedron::kis) but also splits each edge in the middle.
    ///
    /// # Arguments
    ///
    /// * `height` – The offset of the new face centers.
    /// * `vertex_valence_mask` – Only vertices matching the given valences will
    ///   be affected.
    /// * `regular_faces_only` – Only regular faces will be affected.
    pub fn needle(
        &mut self,
        height: Option<Float>,
        vertex_valence_mask: Option<&[usize]>,
        regular_faces_only: Option<bool>,
        change_name: bool,
    ) -> &mut Self {
        // TODO: default
        self.truncate(height, vertex_valence_mask, regular_faces_only, false);
        self.dual(false);

        if change_name {
            let mut params = String::new();
            if let Some(height) = height {
                write!(&mut params, "{}", format_float(height)).unwrap();
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
            self.name = format!("n{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn n(&mut self) -> &mut Self {
        // FIXME: default height
        self.needle(None, None, None, true)
    }
}
