use crate::*;

impl Polyhedron {
    /// Creates the [dual](https://en.wikipedia.org/wiki/Dual_polyhedron).
    /// Replaces each face with a vertex, and each vertex with a face.
    pub fn dual(&mut self, change_name: bool) -> &mut Self {
        let new_positions = face_centers(&self.face_index, &self.positions);
        self.face_index = positions_to_faces(&self.positions, &self.face_index);
        self.positions = new_positions;
        // FIXME: FaceSetIndex

        if change_name {
            self.name = format!("d{}", self.name);
        }

        self
    }

    #[inline]
    pub fn d(&mut self) -> &mut Self {
        self.dual(true)
    }
}
