use crate::*;

const DEFAULT_AMBO_RATIO: Float = 1. / 2.;

impl Polyhedron {
    /// Creates vertices with valence (aka degree) four.
    ///
    /// It is also called [rectification](https://en.wikipedia.org/wiki/Rectification_(geometry)),
    /// or the [medial graph](https://en.wikipedia.org/wiki/Medial_graph) in graph theory.
    pub fn ambo(
        &mut self,
        ratio: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_AMBO_RATIO,
        };

        let edges = self.to_edges();

        let positions: Vec<(&Edge, Point)> = edges
            .par_iter()
            .map(|edge| {
                let edge_positions = index_as_positions(edge, &self.positions);
                (
                    edge,
                    ratio * *edge_positions[0]
                        + (1.0 - ratio) * *edge_positions[1],
                )
            })
            .collect();

        let new_ids = vertex_ids_edge_ref_ref(&positions, 0);

        let face_index: Faces = self
            .face_index
            .par_iter()
            .map(|face| {
                let edges = distinct_face_edges(face);
                edges
                    .iter()
                    .filter_map(|edge| vertex_edge(edge, &new_ids))
                    .collect::<Vec<_>>()
            })
            .chain(
                self.positions
                    // Each old vertex creates a new face ...
                    .par_iter()
                    .enumerate()
                    .map(|(polygon_vertex, _)| {
                        let vertex_number = polygon_vertex as VertexKey;
                        ordered_vertex_edges(
                            vertex_number,
                            &vertex_faces(vertex_number, &self.face_index),
                        )
                        .iter()
                        .map(|ve| {
                            vertex_edge(&distinct_edge(ve), &new_ids).unwrap()
                        })
                        .collect::<Vec<_>>()
                    }),
            )
            .collect();

        self.append_new_face_set(face_index.len());

        self.face_index = face_index;
        self.positions = vertex_values(&positions);

        if change_name {
            let params = match ratio != DEFAULT_AMBO_RATIO {
                true => format_float(ratio),
                false => String::new(),
            };
            self.name = format!("a{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn a(&mut self) -> &mut Self {
        self.ambo(Some(DEFAULT_AMBO_RATIO), true)
    }
}
