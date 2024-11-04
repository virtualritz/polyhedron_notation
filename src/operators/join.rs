use crate::*;

const DEFAULT_JOIN_RATIO: Float = 1. / 2.;

impl Polyhedron {
    /// Creates quadrilateral faces around each original edge. Original
    /// edges are discarded.
    ///
    /// # Arguments
    ///
    /// * `ratio` – The ratio at which the adjacent edges get split. Will be
    ///   clamped to `[0, 1]`. Default value is `0.5`.
    pub fn join(
        &mut self,
        ratio: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_JOIN_RATIO,
        };

        self.dual(false);
        self.ambo(Some(ratio), false);
        self.dual(false);

        if change_name {
            let params = match ratio != DEFAULT_JOIN_RATIO {
                true => format_float(ratio),
                false => String::new(),
            };
            self.name = format!("j{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn j(&mut self) -> &mut Self {
        self.join(Some(DEFAULT_JOIN_RATIO), true)
    }
}
