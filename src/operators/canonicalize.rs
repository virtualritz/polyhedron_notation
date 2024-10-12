use crate::*;

impl Polyhedron {
    /// Apply proper canonicalization. t yypical number of `iterarations` is
    /// `200`+.
    /// FIXME: this is b0rked atm.
    #[inline]
    fn _canonicalize(&mut self, iterations: Option<usize>, change_name: bool) {
        let mut dual = self.clone().dual(false).finalize();

        for _ in 0..iterations.unwrap_or(200) {
            // Reciprocate faces.
            dual.positions =
                _reciprocate_faces(&self.face_index, &self.positions);
            self.positions =
                _reciprocate_faces(&dual.face_index, &dual.positions);
        }

        if change_name {
            let mut params = String::new();
            if let Some(iterations) = iterations {
                write!(&mut params, "{}", iterations).unwrap(); // FIXME
            }
            self.name = format!("N{}{}", params, self.name);
        }
    }
}
