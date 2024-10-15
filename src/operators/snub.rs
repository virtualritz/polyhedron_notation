use crate::*;

const DEFAULT_SNUB_RATIO: Float = 1. / 3.;
const DEFAULT_SNUB_HEIGHT: Float = 1. / 2.;

impl Polyhedron {
    /// Applies a [snub](https://en.wikipedia.org/wiki/Snub_(geometry)) to the shape.
    ///
    /// # Arguments
    ///
    /// * `ratio` – The ratio at which the adjacent edges get split.
    /// * `height` – The height of the newly created centers.
    pub fn snub(
        &mut self,
        ratio: Option<Float>,
        height: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_SNUB_RATIO,
        };
        let height = match height {
            Some(h) => h.clamp(0.0, 1.0),
            None => DEFAULT_SNUB_HEIGHT,
        };

        self.dual(false);
        self.gyro(Some(ratio), Some(height), false);
        self.dual(false);

        if change_name {
            let params = match ratio != DEFAULT_SNUB_RATIO
                || height != DEFAULT_SNUB_HEIGHT
            {
                true => {
                    format!("{},{}", format_float(ratio), format_float(height))
                }
                false => "".to_string(),
            };
            self.name = format!("s{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn s(&mut self) -> &mut Self {
        self.snub(Some(DEFAULT_SNUB_RATIO), Some(DEFAULT_SNUB_HEIGHT), true)
    }
}
