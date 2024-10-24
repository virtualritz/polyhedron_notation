use crate::*;

const DEFAULT_SPHERIZE_STRENGTH: Float = 1.;

impl Polyhedron {
    /// Projects all positions on the unit sphere (at `strength` `1.0`).
    ///
    /// # Arguments
    ///
    /// * `strength` – The strength of the spherization. If `strength` is zero
    ///   this is a no-op and will neither change the geometry nor the name.
    ///   Even if `change_name` is `true`.
    pub fn spherize(
        &mut self,
        strength: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let strength = match strength {
            Some(s) => s,
            None => DEFAULT_SPHERIZE_STRENGTH,
        };

        if 0.0 != strength {
            self.positions.par_iter_mut().for_each(|point| {
                *point =
                    (1.0 - strength) * *point + strength * point.normalized();
            });

            if change_name {
                let params = if strength != DEFAULT_SPHERIZE_STRENGTH {
                    format!("{}", format_float(strength))
                } else {
                    "".to_string()
                };
                self.name = format!("S{}{}", params, self.name);
            }
        }

        self
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn S(&mut self) -> &mut Self {
        self.spherize(None, true)
    }
}
