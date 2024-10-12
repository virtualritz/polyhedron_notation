use crate::*;

impl Polyhedron {
    /// Performs [Catmull-Clark subdivision](https://en.wikipedia.org/wiki/Catmull%E2%80%93Clark_subdivision_surface).
    ///
    /// Each face is replaced with *n* quadralaterals based on edge midpositions
    /// vertices and centroid edge midpositions are average of edge endpositions
    /// and adjacent centroids original vertices replaced by weighted
    /// average of original vertex, face centroids and edge midpositions.
    pub fn catmull_clark_subdivide(&mut self, change_name: bool) -> &mut Self {
        let new_face_vertices = self
            .face_index
            .par_iter()
            .map(|face| {
                let face_positions = index_as_positions(face, &self.positions);
                (face.as_slice(), centroid_ref(&face_positions))
            })
            .collect::<Vec<_>>();

        let edges = self.to_edges();

        let new_edge_vertices = edges
            .par_iter()
            .map(|edge| {
                let ep = index_as_positions(edge, &self.positions);
                let af1 = face_with_edge(edge, &self.face_index);
                let af2 = face_with_edge(&[edge[1], edge[0]], &self.face_index);
                let fc1 =
                    vertex_point(&af1, new_face_vertices.as_slice()).unwrap();
                let fc2 =
                    vertex_point(&af2, new_face_vertices.as_slice()).unwrap();
                (edge, (*ep[0] + *ep[1] + *fc1 + *fc2) * 0.25)
            })
            .collect::<Vec<_>>();

        let new_face_vertex_ids = vertex_ids_ref_ref(
            new_face_vertices.as_slice(),
            self.positions.len() as _,
        );
        let new_edge_vertex_ids = vertex_ids_edge_ref_ref(
            new_edge_vertices.as_slice(),
            (self.positions.len() + new_face_vertices.len()) as _,
        );

        let new_face_index = self
            .face_index
            .par_iter()
            .flat_map(|face| {
                let centroid = vertex(face, &new_face_vertex_ids).unwrap();

                face.iter()
                    .circular_tuple_windows::<(_, _, _)>()
                    .map(|triplet| {
                        let mid1 = vertex_edge(
                            &distinct_edge(&[*triplet.0, *triplet.1]),
                            new_edge_vertex_ids.as_slice(),
                        )
                        .unwrap();
                        let mid2 = vertex_edge(
                            &distinct_edge(&[*triplet.1, *triplet.2]),
                            new_edge_vertex_ids.as_slice(),
                        )
                        .unwrap();
                        vec![centroid, mid1, *triplet.1, mid2]
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let new_positions = self
            .positions
            .par_iter()
            .enumerate()
            .map(|point| {
                let i = point.0 as _;
                let v = point.1;
                let vertex_faces = vertex_faces(i, &self.face_index)
                    .iter()
                    .map(|face| {
                        vertex_point(face, new_face_vertices.as_slice())
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                let n = vertex_faces.len() as Float;
                let f = centroid_ref(&vertex_faces);
                let r = centroid_ref(
                    &vertex_edges(i, &edges)
                        .iter()
                        .map(|edge| {
                            vertex_edge_point(
                                edge,
                                new_edge_vertices.as_slice(),
                            )
                            .unwrap()
                        })
                        .collect::<Vec<_>>(),
                );
                (f + 2.0 * r + (n - 3.0) * *v) / n
            })
            .chain(vertex_values(new_face_vertices.as_slice()))
            .chain(vertex_values(new_edge_vertices.as_slice()))
            .collect::<Points>();

        self.positions = new_positions;
        self.face_index = new_face_index;

        if change_name {
            self.name = format!("v{}", self.name);
        }

        self
    }
}
