use crate::*;

const DEFAULT_QUINTO_HEIGHT: Float = 1. / 2.;

impl Polyhedron {
    /// Splits each edge in the middle and creates new faces in the middle of
    /// each face then connects those.
    ///
    /// # Arguments
    ///
    /// * `height` – The offset of the new faces from the original face.
    pub fn quinto(
        &mut self,
        height: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let height = match height {
            Some(h) => h.clamp(0.0, 3.0),
            None => DEFAULT_QUINTO_HEIGHT,
        };

        let mut new_positions: Vec<(Face, Point)> = self
            .to_edges()
            .par_iter()
            .map(|edge| {
                let edge_positions = index_as_positions(edge, &self.positions);
                (
                    edge.to_vec(),
                    height * (*edge_positions[0] + *edge_positions[1]),
                )
            })
            .collect();

        new_positions.extend(
            self.face_index
                .par_iter()
                .flat_map(|face| {
                    let edge_positions =
                        index_as_positions(face, &self.positions);
                    let centroid = centroid_ref(&edge_positions);
                    (0..face.len())
                        .map(|i| {
                            (
                                extend![..face, i as VertexKey],
                                (*edge_positions[i]
                                    + *edge_positions[(i + 1) % face.len()]
                                    + centroid)
                                    / 3.,
                            )
                        })
                        .collect::<Vec<(Face, Point)>>()
                })
                .collect::<Vec<(Face, Point)>>(),
        );

        let new_ids =
            vertex_ids_ref(&new_positions, self.positions_len() as VertexKey);

        self.positions.extend(vertex_values_as_ref(&new_positions));

        self.face_index = self
            .face_index
            .par_iter()
            .map(|face| {
                (0..face.len())
                    .map(|face_vertex| {
                        vertex(
                            &extend![..face, face_vertex as VertexKey],
                            &new_ids,
                        )
                        .unwrap()
                    })
                    .collect()
            })
            .chain(self.face_index.par_iter().flat_map(|face| {
                (0..face.len())
                    .map(|i| {
                        let v = face[i];
                        let e0 =
                            [face[(i + face.len() - 1) % face.len()], face[i]];
                        let e1 = [face[i], face[(i + 1) % face.len()]];
                        let e0p =
                            vertex(&distinct_edge(&e0), &new_ids).unwrap();
                        let e1p =
                            vertex(&distinct_edge(&e1), &new_ids).unwrap();
                        let iv0 = vertex(
                            &extend![
                                ..face,
                                ((i + face.len() - 1) % face.len())
                                    as VertexKey
                            ],
                            &new_ids,
                        )
                        .unwrap();
                        let iv1 =
                            vertex(&extend![..face, i as VertexKey], &new_ids)
                                .unwrap();
                        vec![v, e1p, iv1, iv0, e0p]
                    })
                    .collect::<Faces>()
            }))
            .collect::<Faces>();

        if change_name {
            let params = match height != DEFAULT_QUINTO_HEIGHT {
                true => format_float(height),
                false => String::new(),
            };
            self.name = format!("q{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn q(&mut self) -> &mut Self {
        self.quinto(Some(DEFAULT_QUINTO_HEIGHT), true)
    }
}
