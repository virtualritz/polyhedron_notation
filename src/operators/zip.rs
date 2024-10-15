use crate::*;

const DEFAULT_ZIP_HEIGHT: Float = 1. / 10.;

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
        let height = match height {
            Some(h) => h.clamp(0.0, 1.0),
            None => DEFAULT_ZIP_HEIGHT,
        };

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
            if height != DEFAULT_ZIP_HEIGHT {
                params = format!("{}", format_float(height));
            }
            if let Some(face_arity_mask) = face_arity_mask {
                params = format!(
                    "{params},{}",
                    format_integer_slice(face_arity_mask)
                );
            }
            if let Some(regular_faces_only) = regular_faces_only {
                if regular_faces_only {
                    params.push_str(",{t}");
                }
            }
            self.name = format!("z{}{}", params, self.name);
        }

        self
    }
}
