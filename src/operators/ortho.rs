use crate::*;

const DEFAULT_ORTHO_RATIO: Float = 0.;

impl Polyhedron {
    /// Connects the center of each face to the center of each edge.
    ///
    /// # Arguments
    ///
    /// * `ratio` – The ratio of the new two parts each original edge is split
    ///   into.
    pub fn ortho(
        &mut self,
        ratio: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_ORTHO_RATIO,
        };

        self.join(Some(ratio), false);
        self.join(Some(ratio), false);

        if change_name {
            let params = match ratio != DEFAULT_ORTHO_RATIO {
                true => format!("{}", format_float(ratio)),
                false => "".to_string(),
            };
            self.name = format!("o{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn o(&mut self) -> &mut Self {
        self.ortho(Some(DEFAULT_ORTHO_RATIO), true)
    }
}
