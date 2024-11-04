use crate::*;

const DEFAULT_INSET_HEIGHT: Float = -1. / 10.;
const DEFAULT_INSET_OFFSET: Float = 3. / 10.;

impl Polyhedron {
    /// Inset faces by `offset` from the original edges.
    ///
    /// # Arguments
    ///
    /// * `offset` – The distance to inset the faces. Default value is `0.3`.
    /// * `face_arity_mask` – Only faces matching the given arities will be
    ///   affected.
    pub fn inset(
        &mut self,
        offset: Option<Float>,
        face_arity_mask: Option<&[usize]>,
        change_name: bool,
    ) -> &mut Self {
        let offset = match offset {
            Some(o) => o.clamp(0.0, 1.0),
            None => DEFAULT_INSET_OFFSET,
        };
        //FIXME: Height parameter and default.

        self.extrude(
            Some(DEFAULT_INSET_HEIGHT),
            Some(offset),
            face_arity_mask,
            false,
        );

        if change_name {
            let params = if let Some(face_arity_mask) = face_arity_mask {
                format!(
                    "{},{}",
                    format_float(offset),
                    format_integer_slice(face_arity_mask)
                )
            } else if offset != DEFAULT_INSET_OFFSET {
                format_float(offset)
            } else {
                String::new()
            };
            self.name = format!("i{}{}", params, self.name);
        }

        self
    }
}
