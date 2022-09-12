//! Window and associated to window rendering context related functions.

use crate::core;
use crate::framework::get_context;
use crate::framework::quad_gl::Color;
use crate::core::PassAction;

// oxid is re-exported for the use in combination with `get_internal_gl`
// pub use oxid;

/// Block execution until the next frame.
pub fn next_frame() -> crate::framework::exec::FrameFuture {
    crate::framework::exec::FrameFuture
}

pub fn clear_background(color: Color) {
    let context = get_context();

    // all drawcalls are batched
    // and batching is not clear-friendly
    // so as a workaround we do immediate render pass with clear color
    let clear = PassAction::clear_color(color.r, color.g, color.b, color.a);
    if let Some(current_pass) = context.draw_context.current_pass {
        context.quad_context.begin_pass(current_pass, clear);
    } else {
        context.quad_context.begin_default_pass(clear);
    }
    context.quad_context.end_render_pass();

    context.draw_context.gl.clear_draw_calls();
}

pub struct InternalGlContext<'a> {
    pub quad_context: &'a mut core::Context,
    pub quad_gl: &'a mut crate::framework::quad_gl::QuadGl,
}

impl<'a> InternalGlContext<'a> {
    /// Draw all the batched stuff and reset the internal state cache
    /// May be helpfull for combining framework's drawing with raw oxid/opengl calls
    pub fn flush(&mut self) {
        let context = get_context();

        context
            .draw_context
            .perform_render_passes(&mut self.quad_context);
    }
}

pub unsafe fn get_internal_gl<'a>() -> InternalGlContext<'a> {
    let context = get_context();

    InternalGlContext {
        quad_context: &mut context.quad_context,
        quad_gl: &mut context.draw_context.gl,
    }
}

pub fn screen_width() -> f32 {
    let context = get_context();

    context.screen_width
}

pub fn screen_height() -> f32 {
    let context = get_context();

    context.screen_height
}
