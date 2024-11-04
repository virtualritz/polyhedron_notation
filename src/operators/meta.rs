use crate::*;

const DEFAULT_META_RATIO: Float = 1. / 2.;
const DEFAULT_META_HEIGHT: Float = 0.;

impl Polyhedron {
    /// Adds vertices at the center and along the edges.
    ///
    /// # Arguments
    ///
    /// * `ratio` – The ratio of the new vertices to the original vertices.
    /// * `height` – The height of the new vertices.
    /// * `vertex_valence_mask` – Only vertices matching the given valences will
    ///   be affected.
    /// * `regular_faces_only` – Only regular faces will be affected.
    pub fn meta(
        &mut self,
        ratio: Option<Float>,
        height: Option<Float>,
        vertex_valence_mask: Option<&[usize]>,
        regular_faces_only: Option<bool>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_META_RATIO,
        };
        let height = match height {
            Some(h) => h.clamp(0.0, 1.0),
            None => DEFAULT_META_HEIGHT,
        };

        self.bevel(
            Some(ratio),
            Some(height),
            vertex_valence_mask,
            regular_faces_only,
            false,
        );
        self.dual(false);

        if change_name {
            let mut params = String::new();
            if ratio != DEFAULT_META_RATIO {
                params = format_float(ratio);
            }
            if height != DEFAULT_META_HEIGHT {
                params = format!("{params},{}", format_float(height));
            }
            if let Some(vertex_valence_mask) = vertex_valence_mask {
                params = format!(
                    "{params},{}",
                    format_integer_slice(vertex_valence_mask)
                );
            }
            if let Some(regular_faces_only) = regular_faces_only {
                if regular_faces_only {
                    params.push_str(",{t}");
                }
            }
            self.name = format!("m{}{}", params, self.name);
        }

        self
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn m(&mut self) -> &mut Self {
        self.meta(None, None, None, Some(false), true)
    }
}
