//! Mose common types that can be glob-imported `use framework::prelude::*` for convenience.

pub use crate::camera::*;
pub use crate::file::*;
pub use crate::input::*;
pub use crate::material::*;
pub use crate::math::*;
pub use crate::models::*;
pub use crate::shapes::*;
pub use crate::text::*;
pub use crate::texture::*;
pub use crate::time::*;
pub use crate::window::*;

pub use glam;
pub use oxid::{conf::Conf, Comparison, PipelineParams, UniformType};
pub use crate::quad_gl::{colors::*, Color, DrawMode, GlPipeline, QuadGl, Vertex};
pub use quad_rand as rand;

pub use crate::collections;
pub use crate::coroutines;

#[cfg(feature = "log-impl")]
pub use crate::logging::*;

pub use crate::color_u8;