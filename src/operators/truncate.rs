use crate::*;

const DEFAULT_TRUNC_HEIGHT: Float = 0.;

impl Polyhedron {
    /// Cuts off the shape at its vertices but leaves a portion of the original
    /// edges.
    ///
    /// # Arguments
    ///
    /// * `height` – The height of the newly created centers.
    /// * `face_arity_mask` - Only faces matching the given arities will be
    ///   affected.
    /// * `regular_faces_only` – Only regular faces will be affected.
    pub fn truncate(
        &mut self,
        height: Option<Float>,
        face_arity_mask: Option<&[usize]>,
        regular_faces_only: Option<bool>,
        change_name: bool,
    ) -> &mut Self {
        let height = match height {
            Some(h) => h.clamp(0.0, 1.0),
            None => DEFAULT_TRUNC_HEIGHT,
        };

        self.dual(false);
        self.kis(
            Some(height),
            face_arity_mask,
            None,
            regular_faces_only,
            false,
        );
        self.dual(false);

        if change_name {
            let mut params = String::new();
            if height != DEFAULT_TRUNC_HEIGHT {
                params = format!("{}", format_float(height))
            }
            if let Some(face_arity_mask) = face_arity_mask {
                params = format!(
                    "{},{}",
                    format_float(height),
                    format_integer_slice(face_arity_mask)
                );
            }
            if let Some(regular_faces_only) = regular_faces_only {
                if regular_faces_only {
                    params.push_str(",{t}");
                }
            }
            self.name = format!("t{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn t(&mut self) -> &mut Self {
        self.truncate(Some(DEFAULT_TRUNC_HEIGHT), None, None, true)
    }
}
