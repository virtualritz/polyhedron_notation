#[cfg(feature = "f64")]
mod f64;
#[cfg(feature = "f64")]
pub use crate::types::f64::*;

#[cfg(not(feature = "f64"))]
mod f32;
#[cfg(not(feature = "f64"))]
pub use crate::types::f32::*;

pub type VertexKey = u32;
pub type FaceKey = u32;
pub type Face = Vec<VertexKey>;
pub(crate) type FaceSlice = [VertexKey];
pub type Faces = Vec<Face>;
pub(crate) type FacesSlice = [Face];
pub type FaceSet = Vec<VertexKey>;
pub type Edge = [VertexKey; 2];
pub type Edges = Vec<Edge>;
pub type EdgesSlice = [Edge];
pub(crate) type _EdgeSlice = [Edge];
#[allow(dead_code)]
pub type Normals = Vec<Normal>;
pub type Points = Vec<Point>;
pub(crate) type PointsSlice = [Point];
pub(crate) type PointsRefSlice<'a> = [&'a Point];

pub trait Precision {
    fn precision(&self, decimal: u32) -> Self;
}

impl Precision for Float {
    fn precision(&self, decimal: u32) -> Self {
        let p = 10u32.pow(decimal) as Float;
        (*self * p).round() / p
    }
}

impl Precision for Vector {
    fn precision(&self, decimal: u32) -> Self {
        Vector::new(
            self.x.precision(decimal),
            self.y.precision(decimal),
            self.z.precision(decimal),
        )
    }
}
