use crate::*;

// NOTE: conway2.scad uses ratio of 1. / 3.
const DEFAULT_CHAMFER_RATIO: Float = 1. / 2.;
const MAX_CHAMFER_RATIO: Float = 1.;
const MIN_CHAMFER_RATIO: Float = 0.;

impl Polyhedron {
    /// [Chamfers](https://en.wikipedia.org/wiki/Chamfer_(geometry)) edges.
    /// I.e. adds a new hexagonal face in place of each original edge.
    ///
    /// # Arguments
    ///
    /// * `ratio` - The ratio of the new faces to the old faces.
    pub fn chamfer(
        &mut self,
        ratio: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(MIN_CHAMFER_RATIO, MAX_CHAMFER_RATIO),
            None => DEFAULT_CHAMFER_RATIO,
        }; // FIXME: helper

        let new_positions: Vec<(Face, Point)> = self
            .face_index
            .par_iter()
            .flat_map(|face| {
                let face_positions = index_as_positions(face, &self.positions);
                let centroid = centroid_ref(&face_positions);
                let mut result = Vec::new();
                face.iter().enumerate().for_each(|face_point| {
                    let j = face_point.0;
                    let mut new_face = face.clone();
                    new_face.push(face[j]);
                    result.push((
                        new_face,
                        *face_positions[j]
                            + ratio * (centroid - *face_positions[j]),
                    ))
                });
                result
            })
            .collect();

        let new_ids =
            vertex_ids_ref(&new_positions, self.positions_len() as VertexKey);

        let face_index: Faces = self
            .face_index
            .par_iter()
            .map(|face| {
                // FIXME: use iterators with double collect
                let mut new_face = Vec::with_capacity(face.len());
                face.iter().for_each(|vertex_key| {
                    let mut face_key = face.clone();
                    face_key.push(*vertex_key);
                    new_face.push(vertex(&face_key, &new_ids).unwrap());
                });
                new_face
            })
            .chain(self.face_index.par_iter().flat_map(|face| {
                face.iter()
                    .circular_tuple_windows::<(_, _, _)>()
                    .filter_map(|v| {
                        if v.0 < v.1 {
                            let a: VertexKey = *v.0;
                            let b: VertexKey = *v.1;
                            let opposite_face =
                                face_with_edge(&[b, a], &self.face_index);
                            Some(vec![
                                a,
                                vertex(&extend![..opposite_face, a], &new_ids)
                                    .unwrap(),
                                vertex(&extend![..opposite_face, b], &new_ids)
                                    .unwrap(),
                                b,
                                vertex(&extend![..face, b], &new_ids).unwrap(),
                                vertex(&extend![..face, a], &new_ids).unwrap(),
                            ])
                        } else {
                            None
                        }
                    })
                    .collect::<Faces>()
            }))
            .collect::<Faces>();

        self.append_new_face_set(face_index.len());

        self.face_index = face_index;
        self.positions.par_iter_mut().for_each(|point| {
            // NOTE: Calculation based on update in conway2.scad
            *point = (1.0 - ratio) * *point;
        });
        self.positions.extend(vertex_values(&new_positions));

        if change_name {
            let params = match ratio != DEFAULT_CHAMFER_RATIO {
                true => format_float(ratio),
                false => "".to_string(),
            };
            self.name = format!("c{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn c(&mut self) -> &mut Self {
        self.chamfer(Some(DEFAULT_CHAMFER_RATIO), true)
    }
}
