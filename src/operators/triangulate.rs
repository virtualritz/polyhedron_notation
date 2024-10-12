use crate::*;

/// # Triangulation
impl Polyhedron {
    #[inline]
    /// Bitriangulates quadrilateral faces.
    ///
    /// N-gon trinagulation is naive and may yield inferor results.
    ///
    /// # Arguments
    ///
    /// * `shortest` - If `true`, use shortest diagonal so triangles are most
    ///   nearly equilateral. On by default.
    pub fn triangulate(&mut self, shortest: Option<bool>) -> &mut Self {
        self.face_index = self
            .face_index
            .par_iter()
            .flat_map(|face| match face.len() {
                // Bitriangulate quadrilateral faces use shortest diagonal so
                // triangles are most nearly equilateral.
                4 => {
                    let p = index_as_positions(face, &self.positions);

                    if shortest.unwrap_or(true)
                        == ((*p[0] - *p[2]).mag_sq() < (*p[1] - *p[3]).mag_sq())
                    {
                        vec![
                            vec![face[0], face[1], face[2]],
                            vec![face[0], face[2], face[3]],
                        ]
                    } else {
                        vec![
                            vec![face[1], face[2], face[3]],
                            vec![face[1], face[3], face[0]],
                        ]
                    }
                }
                5 => vec![
                    vec![face[0], face[1], face[4]],
                    vec![face[1], face[2], face[4]],
                    vec![face[4], face[2], face[3]],
                ],
                // FIXME: a nicer way to triangulate n-gons.
                _ => {
                    let a = face[0];
                    let mut bb = face[1];
                    face.iter()
                        .skip(2)
                        .map(|c| {
                            let b = bb;
                            bb = *c;
                            vec![a, b, *c]
                        })
                        .collect()
                }
            })
            .collect();

        self
    }
}
