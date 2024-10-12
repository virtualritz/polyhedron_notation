use crate::*;

impl Polyhedron {
    /// [Reflects](https://en.wikipedia.org/wiki/Reflection_(mathematics)) the shape.
    pub fn reflect(&mut self, change_name: bool) -> &mut Self {
        self.positions = self
            .positions
            .par_iter()
            .map(|v| Point::new(v.x, -v.y, v.z))
            .collect();
        self.reverse();

        if change_name {
            self.name = format!("r{}", self.name);
        }

        self
    }

    #[inline]
    pub fn r(&mut self) -> &mut Self {
        self.reflect(true)
    }
}
