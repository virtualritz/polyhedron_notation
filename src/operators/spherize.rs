use crate::*;

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
        // TODO: Default
        let strength_ = strength.unwrap_or(1.0);

        if 0.0 != strength_ {
            self.positions.par_iter_mut().for_each(|point| {
                *point =
                    (1.0 - strength_) * *point + strength_ * point.normalized();
            });

            if change_name {
                let mut params = String::new();
                if let Some(strength) = strength {
                    write!(&mut params, "{}", format_float(strength)).unwrap();
                }
                self.name = format!("S{}{}", params, self.name);
            }
        }

        self
    }
}
