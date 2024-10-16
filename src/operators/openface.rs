use crate::*;

impl Polyhedron {
    #[allow(clippy::too_many_arguments)]
    fn _open_face(
        &self,
        outer_inset_ratio: Option<Float>,
        outer_inset: Option<Float>,
        inner_inset_ratio: Option<Float>,
        inner_inset: Option<Float>,
        depth: Option<Float>,
        face_arity: Option<&[usize]>,
        min_edge_length: Option<Float>,
        _no_cut: Option<bool>,
    ) {
        // upper and lower inset can be specified by ratio or absolute distance
        //  let(inner_inset_ratio= inner_inset_ratio == undef ?
        // outer_inset_ratio : inner_inset_ratio,

        //pf=p_faces(obj),
        //pv=p_vertices(obj))

        // Corresponding positions on inner surface.
        let inverse_positions = self
            .positions
            .iter()
            .enumerate()
            .map(|point| {
                let vertex_faces = vertex_faces(point.0 as _, &self.face_index);
                // Calculate average normal at vertex.
                let average_normal_ref = vertex_faces
                    .iter()
                    .map(|face| {
                        average_normal_ref(&index_as_positions(
                            face,
                            &self.positions,
                        ))
                    })
                    .fold(Normal::zero(), |accumulate, normal| {
                        accumulate + normal
                    })
                    / vertex_faces.len() as Float;

                *point.1 + depth.unwrap_or(0.2) * average_normal_ref
            })
            .collect::<Vec<_>>();

        let _new_vertices = self
            .face_index
            .iter()
            // Filter out faces that have an unwanted arity or are too small.
            .filter(|face| {
                face_arity_matches(face, face_arity)
                    && _minimal_edge_length(face, &self.positions)
                        > min_edge_length.unwrap_or(0.01)
            })
            .flat_map(|face| {
                let face_positions = index_as_positions(face, &self.positions);
                let ofp = index_as_positions(face, &inverse_positions);
                let c = centroid_ref(&face_positions);
                let oc = centroid_ref(&ofp);

                face.iter()
                    .enumerate()
                    .flat_map(|f| {
                        let _v = *f.1;
                        let p = face_positions[f.0];
                        let p1 = face_positions[(f.0 + 1) % face.len()];
                        let p0 =
                            face_positions[(f.0 + face.len() - 1) % face.len()];

                        let sa = _angle_between(&(*p0 - *p), &(*p1 - *p), None);
                        let bv = 0.5
                            * ((*p1 - *p).normalized()
                                + (*p0 - *p).normalized());
                        let op = ofp[f.0];

                        let _ip = match outer_inset {
                            None => {
                                *p + (c - *p) * outer_inset_ratio.unwrap_or(0.2)
                            }
                            Some(outer_inset) => {
                                *p + outer_inset / sa.sin() * bv
                            }
                        };
                        let _oip = match inner_inset {
                            None => {
                                *op + (oc - *op)
                                    * inner_inset_ratio.unwrap_or(0.2)
                            }
                            Some(inner_inset) => {
                                *op + inner_inset / sa.sin() * bv
                            }
                        };
                        //vec![[[face, v], ip], [[face, -v - 1], oip]]
                        vec![]
                    })
                    .collect::<Vec<_>>()
                //vec![]
            })
            .collect::<Vec<Point>>();
        /*
        // the inset positions on outer and inner surfaces
        // outer inset positions keyed by face, v, inner positions by face,-v-1
                flatten(
                  [ for (face = pf)
                    if(face_arity_matches(face,fn)
                       && min_edge_length(face,pv) > min_edge_length)
                        let(fp=as_positions(face,pv),
                            ofp=as_positions(face,inv),
                            c=centroid(fp),
                            oc=centroid(ofp))

                        flatten(
                           [for (i=[0:len(face)-1])
                            let(v=face[i],
                                p = fp[i],
                                p1= fp[(i+1)%len(face)],
                                p0=fp[(i-1 + len(face))%len(face)],
                                sa = angle_between(p0-p,p1-p),
                                bv = (unitv(p1-p)+unitv(p0-p))/2,
                                op= ofp[i],
                                ip = outer_inset ==  undef
                                    ? p + (c-p)*outer_inset_ratio
                                    : p + outer_inset/sin(sa) * bv ,
                                oip = inner_inset == undef
                                    ? op + (oc-op)*inner_inset_ratio
                                    : op + inner_inset/sin(sa) * bv)
                            [ [[face,v],ip],[[face,-v-1],oip]]
                           ])
                    ])
                  )
          let(newids=vertex_ids(newv,2*len(pv)))
          let(newf =
                flatten(
                 [ for (i = [0:len(pf)-1])
                   let(face = pf[i])
                   flatten(
                     face_arity_matches(face,fn)
                       && min_edge_length(face,pv) > min_edge_length
                       && i  >= nocut

                       ? [for (j=[0:len(face)-1])   //  replace N-face with 3*N quads
                         let (a=face[j],
                              inseta = vertex([face,a],newids),
                              oinseta= vertex([face,-a-1],newids),
                              b=face[(j+1)%len(face)],
                              insetb= vertex([face,b],newids),
                              oinsetb=vertex([face,-b-1],newids),
                              oa=len(pv) + a,
                              ob=len(pv) + b)

                            [
                              [a,b,insetb,inseta]  // outer face
                             ,[inseta,insetb,oinsetb,oinseta]  //wall
                             ,[oa,oinseta,oinsetb,ob]  // inner face
                            ]
                          ]
                       :  [[face],  //outer face
                           [reverse([  //inner face
                                  for (j=[0:len(face)-1])
                                  len(pv) +face[j]
                                ])
                           ]
                          ]
                      )
                ] ))

          poly(name=str("L",p_name(obj)),
              vertices=  concat(pv, inv, vertex_values(newv)) ,
              faces= newf,
              debug=newv
              )
           ; // end openface
           */
    }
}
