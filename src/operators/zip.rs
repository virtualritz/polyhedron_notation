use crate::*;

impl Polyhedron {
    /// [Bitruncates](https://en.wikipedia.org/wiki/Bitruncation) the shape.
    ///
    /// # Arguments
    ///
    /// * `height` – The height offset of the newly created vertices.
    /// * `face_arity_mask` – Only faces with the given arity will be affected.
    /// * `regular_faces_only` – Only regular faces will be affected.
    pub fn zip(
        &mut self,
        height: Option<Float>,
        face_arity_mask: Option<&[usize]>,
        regular_faces_only: Option<bool>,
        change_name: bool,
    ) -> &mut Self {
        // TODO
        self.kis(height, face_arity_mask, None, regular_faces_only, false);
        self.dual(false);

        if change_name {
            let mut params = String::new();
            if let Some(height) = height {
                write!(&mut params, "{}", format_float(height)).unwrap();
            }
            if let Some(face_arity_mask) = face_arity_mask {
                write!(
                    &mut params,
                    ",{}",
                    format_integer_slice(face_arity_mask)
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
            self.name = format!("z{}{}", params, self.name);
        }

        self
    }
}
