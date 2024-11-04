use crate::*;

const DEFAULT_EXPAND_HEIGHT: Float = 1. / 2.;

impl Polyhedron {
    /// [Cantellates](https://en.wikipedia.org/wiki/Cantellation_(geometry)).
    /// I.e. creates a new facet in place of each edge and of each vertex.
    ///
    /// # Arguments
    ///
    /// * `ratio` - The ratio of the new faces to the old faces.
    pub fn expand(
        &mut self,
        height: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let height = match height {
            Some(h) => h,
            None => DEFAULT_EXPAND_HEIGHT,
        };

        self.ambo(Some(height), false);
        self.ambo(Some(height), false);

        if change_name {
            let params = match height != DEFAULT_EXPAND_HEIGHT {
                true => format_float(height),
                false => String::new(),
            };
            self.name = format!("e{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn e(&mut self) -> &mut Self {
        self.expand(Some(DEFAULT_EXPAND_HEIGHT), true)
    }
}
