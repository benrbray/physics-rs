use std::rc::Rc;
use glow::{Context, HasContext};
use bevy_ecs::prelude::*;

use crate::game_bevy::components;
use crate::{graphics::{batch_poly_renderer::BatchPolyRenderer, shader::Shader}};

/* ---------------------------------- */

// custom NonSend resource
// https://bevy-cheatbook.github.io/programming/non-send.html#custom-non-send-resources

pub struct RenderResource {
  pub gl: Rc<Context>,
  pub shape_renderer: BatchPolyRenderer,
  pub aabb_renderer: BatchPolyRenderer
}

// TODO where does this code belong?
fn activate_shaders(gl: Rc<Context>) {
    /* ---- compile shaders ---- */

    let vert_src = include_str!("../../../shaders/basic/basic.vert");
    let frag_src = include_str!("../../../shaders/basic/basic.frag");
    let shader = Shader::build(&gl, vert_src, frag_src, &[]).unwrap();

    shader.activate(&gl);
}

impl RenderResource {
  pub fn build(gl: Rc<Context>) -> RenderResource {
    // batch renderers
    let shape_renderer = BatchPolyRenderer::build(Rc::clone(&gl));
    let aabb_renderer = BatchPolyRenderer::build(Rc::clone(&gl));

    activate_shaders(Rc::clone(&gl));

    return RenderResource {
      gl,
      shape_renderer,
      aabb_renderer
    }
  }

  pub fn _render_init(_gl: Rc<Context>) {
    // single draw call
    // https://registry.khronos.org/webgl/specs/latest/2.0/#5.18
    // self.gl.enable(glow::PRIMITIVE_RESTART_FIXED_INDEX);

    // TODO (Ben @ 2024/08/25) WebGL enables this by default and errors if we
    // try to enable it.  So, the enable should only be called in desktop builds.
  }

  pub fn render_begin(&self) {
    unsafe {
      self.gl.clear(glow::COLOR_BUFFER_BIT);
    }
  }

  fn render_end(&self) {
    // TODO double buffering only for desktop target, as WebGL does it automatically
    // self.window.gl_swap_window();
  }

  fn render_shapes(&self, data: &Query<RenderData>) {
    // TODO (Ben @ 2024/08/25) optimize by reusing these vectors?
    let mut vbo_data = Vec::<f32>::new();
    let mut ebo_data = Vec::<u32>::new();

    let mut max_vbo_idx: u32 = 0;
    let mut num_shapes = 0;
    for (pos, geom, _) in data {
      for col in geom.shape.points.column_iter() {
        vbo_data.push(pos.pos.0 + col.x);
        vbo_data.push(pos.pos.1 + col.y);

        ebo_data.push(max_vbo_idx);
        max_vbo_idx += 1;
      }

      ebo_data.push(u32::MAX); // PRIMITIVE_RESTART_FIXED_INDEX
      num_shapes += 1;
    }

    self.shape_renderer.render(vbo_data, ebo_data, num_shapes, max_vbo_idx, true, true);
  }

  fn render_aabb(&self, data: &Query<RenderData>) {
    // TODO (Ben @ 2024/08/25) optimize by reusing these vectors?
    let mut vbo_data = Vec::<f32>::new();
    let mut ebo_data = Vec::<u32>::new();

    let mut max_vbo_idx: u32 = 0;
    let mut num_shapes = 0;
    for (pos, _, collider) in data {
      // bottom left
      vbo_data.push(pos.pos.0 + collider.volume.lower_bound.x);
      vbo_data.push(pos.pos.1 + collider.volume.lower_bound.y);
      ebo_data.push(max_vbo_idx);
      max_vbo_idx += 1;

      // top left
      vbo_data.push(pos.pos.0 + collider.volume.lower_bound.x);
      vbo_data.push(pos.pos.1 + collider.volume.upper_bound.y);
      ebo_data.push(max_vbo_idx);
      max_vbo_idx += 1;

      // top right
      vbo_data.push(pos.pos.0 + collider.volume.upper_bound.x);
      vbo_data.push(pos.pos.1 + collider.volume.upper_bound.y);
      ebo_data.push(max_vbo_idx);
      max_vbo_idx += 1;

      // bottom right
      vbo_data.push(pos.pos.0 + collider.volume.upper_bound.x);
      vbo_data.push(pos.pos.1 + collider.volume.lower_bound.y);
      ebo_data.push(max_vbo_idx);
      max_vbo_idx += 1;

      // end shape
      ebo_data.push(u32::MAX); // PRIMITIVE_RESTART_FIXED_INDEX
      num_shapes += 1;
    }

    self.aabb_renderer.render(vbo_data, ebo_data, num_shapes, max_vbo_idx, false, true);
  }
}

type RenderData<'a> = (
  &'a components::Position,
  &'a components::Geom2d,
  &'a components::Collider
);

// renderer must always run on main thread
// so we use bevy's NonSend
// https://bevy-cheatbook.github.io/programming/non-send.html
pub fn render_system(
  data: Query<RenderData>,
  renderer: NonSend<RenderResource>
) {
  renderer.render_begin();
  renderer.render_shapes(&data);
  renderer.render_aabb(&data);
  renderer.render_end();
}