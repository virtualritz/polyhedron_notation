use crate::*;

const DEFAULT_BEVEL_RATIO: Float = 1. / 2.;
const DEFAULT_BEVEL_HEIGHT: Float = 0.;

impl Polyhedron {
    /// Adds faces at the center, original vertices, and along the edges.
    ///
    /// # Arguments
    ///
    /// * `ratio` - The ratio of the new vertices to the original vertices.
    /// * `height` - The height (depth) of the bevel.
    /// * `face_arity_mask` - Only faces matching the given arities will be
    ///   affected.
    /// * `regular_faces_only` - Only regular faces will be affected.
    pub fn bevel(
        &mut self,
        ratio: Option<Float>,
        height: Option<Float>,
        face_arity_mask: Option<&[usize]>,
        regular_faces_only: Option<bool>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_BEVEL_RATIO,
        };
        let height = match height {
            Some(h) => h.clamp(0.0, 1.0),
            None => DEFAULT_BEVEL_HEIGHT,
        };

        self.ambo(Some(ratio), false);
        self.truncate(Some(height), face_arity_mask, regular_faces_only, false);

        if change_name {
            let mut params = String::new();
            if ratio != DEFAULT_BEVEL_RATIO {
                write!(&mut params, "{}", format_float(ratio)).unwrap();
            }
            if height != DEFAULT_BEVEL_HEIGHT {
                write!(&mut params, ",{}", format_float(height)).unwrap();
            } else {
                write!(&mut params, ",").unwrap();
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
                write!(&mut params, ",").unwrap(); // FIXME
            }
            params = params.trim_end_matches(',').to_string();
            self.name = format!("b{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn b(&mut self) -> &mut Self {
        self.bevel(None, None, None, None, true)
    }
}
