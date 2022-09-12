//! Mose common types that can be glob-imported `use oxid::framework::prelude::*` for convenience.

pub use crate::framework::camera::*;
pub use crate::framework::file::*;
pub use crate::framework::input::*;
pub use crate::framework::material::*;
pub use crate::framework::math::*;
pub use crate::framework::models::*;
pub use crate::framework::shapes::*;
pub use crate::framework::text::*;
pub use crate::framework::texture::*;
pub use crate::framework::time::*;
pub use crate::framework::window::*;

pub use crate::framework::quad_gl::{colors::*, Color, DrawMode, GlPipeline, QuadGl, Vertex};
pub use crate::core::{conf::Conf, Comparison, PipelineParams, UniformType};
pub use glam;
pub use quad_rand as rand;

pub use crate::framework::collections;
pub use crate::framework::coroutines;
// pub use crate::framework::quad_gl::color_u8;
