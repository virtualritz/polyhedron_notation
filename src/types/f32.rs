pub type Float = f32;
pub type Vector = ultraviolet::vec::Vec3;
pub type Normal = Vector;
pub type Point = Vector;

pub const PARTIAL_EQ_PRECISION: u32 = 4u32;

#[allow(dead_code)]
pub const FLOAT_SIZE: usize = size_of::<f32>();

#[cfg(feature = "bevy")]
pub type RenderFloat = f32;
