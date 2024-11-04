use crate::*;

const DEFAULT_PLANE_ITER: usize = 10;

impl Polyhedron {
    /// Applies quick and dirty canonicalization.
    ///
    /// # Arguments
    ///
    /// * `iterations` – The number of iterations to perform. Typical number of
    ///   `iterations are `100`+. The default is `100`.
    #[inline]
    pub fn planarize(
        &mut self,
        iterations: Option<usize>,
        change_name: bool,
    ) -> &mut Self {
        let iterations = match iterations {
            Some(i) => i.clamp(0, 100_000),
            None => DEFAULT_PLANE_ITER,
        };
        let mut dual = self.clone().dual(false).finalize();

        for _ in 0..iterations {
            // Reciprocate face centers.
            dual.positions =
                reciprocate_face_centers(&self.face_index, &self.positions);
            self.positions =
                reciprocate_face_centers(&dual.face_index, &dual.positions);
        }

        if change_name {
            let params = match iterations != DEFAULT_PLANE_ITER {
                true => iterations.to_string(),
                false => String::new(),
            };
            self.name = format!("K{}{}", params, self.name);
        }

        self
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn K(&mut self) -> &mut Self {
        self.planarize(Some(DEFAULT_PLANE_ITER), true)
    }
}
