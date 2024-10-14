pub type Float = f64;
pub type Vector = ultraviolet::vec::DVec3;
pub type Normal = Vector;
pub type Point = Vector;

pub const PARTIAL_EQ_PRECISION: u32 = 8u32;

#[allow(dead_code)]
pub const FLOAT_SIZE: usize = size_of::<f64>();

#[cfg(feature = "bevy")]
pub type RenderFloat = f32;
