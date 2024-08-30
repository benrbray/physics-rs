use std::rc::Rc;

use specs::prelude::*;
use glow::HasContext;
use crate::{game::components, graphics::batch_poly_renderer::BatchPolyRenderer};

////////////////////////////////////////////////////////////////////////////////

pub struct RenderSystem {
  gl:  Rc<glow::Context>,
  shape_renderer: BatchPolyRenderer,
  aabb_renderer: BatchPolyRenderer
}

// resources required for execution
#[derive(SystemData)]
pub struct RenderSystemData<'a> {
  geometry: ReadStorage<'a, components::Geom2d>,
  collider: ReadStorage<'a, components::Collider>,
  position: ReadStorage<'a, components::Position>
}

impl RenderSystem {
  pub fn build(
    gl: Rc<glow::Context>,
  ) -> RenderSystem {
    let shape_renderer = BatchPolyRenderer::build(Rc::clone(&gl));
    let aabb_renderer = BatchPolyRenderer::build(Rc::clone(&gl));

    let system = RenderSystem {
      gl,
      shape_renderer,
      aabb_renderer
    };

    system.render_init();

    system
  }
}

impl<'b> System<'b> for RenderSystem {
  type SystemData = RenderSystemData<'b>;

  // plan:
  // 1. batch all 2d convex shapes into single vbo, single draw call
  //    (repopulate entire vbo per frame) 
  // 2. re-populate entire vbo per-frame, double-buffered
  //    to encourage GPU pipelining
  //    https://web.archive.org/web/20200622011519/https://community.arm.com/developer/tools-software/graphics/b/blog/posts/mali-performance-6-efficiently-updating-dynamic-resources
  // 3. use GL_PRIMITIVE_RESTART_FIXED_INDEX so that multiple triangle
  //    strips can be drawn from a single vbo
  //
  // hints:

  //   GL_DYNAMIC_DRAW?
  //   glBufferSubData
  //   buffer orphaning
  //   https://www.khronos.org/opengl/wiki/Buffer_Object_Streaming
  //   https://old.reddit.com/r/opengl/comments/1461fzc/vertex_buffer_streaming_techniques_comparision/

  fn run(&mut self, data: RenderSystemData) {
    self.render_begin();
    self.render_shapes(&data);
    self.render_aabb(&data);
    self.render_end();
  }
}

impl RenderSystem {
  fn render_init(&self) {
    // single draw call
    // https://registry.khronos.org/webgl/specs/latest/2.0/#5.18
    // self.gl.enable(glow::PRIMITIVE_RESTART_FIXED_INDEX);

    // TODO (Ben @ 2024/08/25) WebGL enables this by default and errors if we
    // try to enable it.  So, the enable should only be called in desktop builds.
  }

  fn render_begin(&self) {
    unsafe {
      self.gl.clear(glow::COLOR_BUFFER_BIT);
    }
  }

  fn render_end(&self) {
    // TODO double buffering only for desktop target, as WebGL does it automatically
    // self.window.gl_swap_window();
  }

  fn render_shapes(&self, data: &RenderSystemData) {
    // TODO (Ben @ 2024/08/25) optimize by reusing these vectors?
    let mut vbo_data = Vec::<f32>::new();
    let mut ebo_data = Vec::<u32>::new();

    let mut max_vbo_idx: u32 = 0;
    let mut num_shapes = 0;
    for (geom, pos) in (&data.geometry, &data.position).join() {
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

  fn render_aabb(&self, data: &RenderSystemData) {
    // TODO (Ben @ 2024/08/25) optimize by reusing these vectors?
    let mut vbo_data = Vec::<f32>::new();
    let mut ebo_data = Vec::<u32>::new();

    let mut max_vbo_idx: u32 = 0;
    let mut num_shapes = 0;
    for (collider, pos) in (&data.collider, &data.position).join() {
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