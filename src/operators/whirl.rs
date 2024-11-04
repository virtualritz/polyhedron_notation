use crate::*;

const DEFAULT_WHIRL_RATIO: Float = 1. / 3.;
const DEFAULT_WHIRL_HEIGHT: Float = 1. / 5.;

impl Polyhedron {
    /// Splits each edge into three parts and connects the new vertices. But
    /// also splits the newly formed connections and connects those.
    ///
    /// # Arguments
    ///
    /// * `ratio` – The ratio at which the adjacent edges get split.
    /// * `height` – The height offset of the newly created vertices.
    pub fn whirl(
        &mut self,
        ratio: Option<Float>,
        height: Option<Float>,
        change_name: bool,
    ) -> &mut Self {
        let ratio = match ratio {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_WHIRL_RATIO,
        };
        let height = match height {
            Some(r) => r.clamp(0.0, 1.0),
            None => DEFAULT_WHIRL_HEIGHT,
        };

        let new_positions: Vec<(Face, Point)> = self
            .face_index
            .par_iter()
            .flat_map(|face| {
                let face_positions = index_as_positions(face, &self.positions);
                let center = centroid_ref(&face_positions)
                    + average_normal_ref(&face_positions) * height;
                face.iter()
                    .enumerate()
                    .map(|v| {
                        let edge_positions = [
                            face_positions[v.0],
                            face_positions[(v.0 + 1) % face.len()],
                        ];
                        let middle: Point = *edge_positions[0]
                            + ratio * (*edge_positions[1] - *edge_positions[0]);
                        (
                            extend![..face, *v.1],
                            middle + ratio * (center - middle),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .chain(self.to_edges().par_iter().flat_map(|edge| {
                let edge_positions = index_as_positions(edge, &self.positions);
                vec![
                    (
                        edge.to_vec(),
                        *edge_positions[0]
                            + ratio * (*edge_positions[1] - *edge_positions[0]),
                    ),
                    (
                        vec![edge[1], edge[0]],
                        *edge_positions[1]
                            + ratio * (*edge_positions[0] - *edge_positions[1]),
                    ),
                ]
            }))
            .collect();

        let new_ids =
            vertex_ids_ref(&new_positions, self.positions_len() as VertexKey);

        self.positions.extend(vertex_values(&new_positions));

        let old_face_index_len = self.face_index.len();

        self.face_index = self
            .face_index
            .par_iter()
            .flat_map(|face| {
                face.iter()
                    .circular_tuple_windows::<(_, _, _)>()
                    .map(|v| {
                        let edeg_ab = vertex(&[*v.0, *v.1], &new_ids).unwrap();
                        let edeg_ba = vertex(&[*v.1, *v.0], &new_ids).unwrap();
                        let edeg_bc = vertex(&[*v.1, *v.2], &new_ids).unwrap();
                        let mut mid = face.clone();
                        mid.push(*v.0);
                        let mid_a = vertex(&mid, &new_ids).unwrap();
                        mid.pop();
                        mid.push(*v.1);
                        let mid_b = vertex(&mid, &new_ids).unwrap();
                        vec![edeg_ab, edeg_ba, *v.1, edeg_bc, mid_b, mid_a]
                    })
                    .collect::<Faces>()
            })
            .chain(self.face_index.par_iter().map(|face| {
                let mut new_face = face.clone();
                face.iter()
                    .map(|a| {
                        new_face.push(*a);
                        let result = vertex(&new_face, &new_ids).unwrap();
                        new_face.pop();
                        result
                    })
                    .collect()
            }))
            .collect::<Faces>();

        self.append_new_face_set(self.face_index.len() - old_face_index_len);

        if change_name {
            let params = match ratio != DEFAULT_WHIRL_RATIO
                || height != DEFAULT_WHIRL_HEIGHT
            {
                true => {
                    format!("{},{}", format_float(ratio), format_float(height))
                }
                false => String::new(),
            };
            self.name = format!("w{}{}", params, self.name);
        }

        self
    }

    #[inline]
    pub fn w(&mut self) -> &mut Self {
        self.whirl(Some(DEFAULT_WHIRL_RATIO), Some(DEFAULT_WHIRL_HEIGHT), true)
    }
}
