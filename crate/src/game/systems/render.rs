use std::rc::Rc;

use specs::prelude::*;
use glow::HasContext;
use crate::game::components;

pub struct RenderSystem {
  gl:  Rc<glow::Context>,
  vbo: glow::Buffer,
  ebo: glow::Buffer,
  vao: glow::VertexArray,
}

impl Drop for RenderSystem {
  fn drop(&mut self) {
    unsafe {
      self.gl.delete_vertex_array(self.vao);
      self.gl.delete_buffer(self.vbo);
    }
  }
}

// resources required for execution
#[derive(SystemData)]
pub struct RenderSystemData<'a> {
  geometry: ReadStorage<'a, components::Geom2d>,
  position: ReadStorage<'a, components::Position>
}

impl RenderSystem {
  pub fn build(
    gl: Rc<glow::Context>,
  ) -> RenderSystem {
    // vertex buffer object
    let vbo;
    unsafe {
      vbo = gl.create_buffer().unwrap();
	    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    }

    // We now construct a vertex array to describe the format of the input buffer
    let vao;
    unsafe {
      vao = gl.create_vertex_array().unwrap();
      gl.bind_vertex_array(Some(vao));
      gl.enable_vertex_attrib_array(0);
      let num_bytes: i32 = (2 * core::mem::size_of::<f32>()).try_into().unwrap();
      gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, num_bytes, 0);
    }

    // element buffer object (requires bound VAO)
    let ebo;
    unsafe {
      ebo = gl.create_buffer().unwrap();
	    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
    }

    RenderSystem {
      gl,
      vbo,
      ebo,
      vao,
    }
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

      ebo_data.push(u32::MAX);
      num_shapes += 1;
    }

    let vbo_data_u8;
    let ebo_data_u8;
    unsafe {
      vbo_data_u8 = convert_f32_u8(&vbo_data);
      ebo_data_u8 = convert_u32_u8(&ebo_data);
    }

    unsafe {
      self.gl.clear(glow::COLOR_BUFFER_BIT);

      // write new vertex buffer
      self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
      self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vbo_data_u8, glow::DYNAMIC_DRAW);

      // write new element buffer
      self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
      self.gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, ebo_data_u8, glow::DYNAMIC_DRAW);

      // single draw call
      // self.gl.enable(glow::PRIMITIVE_RESTART_FIXED_INDEX);
      self.gl.enable(glow::BLEND);

      let ebo_len = max_vbo_idx + num_shapes;
      self.gl.draw_elements(glow::TRIANGLE_FAN, ebo_len.try_into().unwrap(), glow::UNSIGNED_INT, 0);

      self.gl.blend_func(glow::SRC_COLOR, glow::ONE_MINUS_SRC_COLOR);

      // outlines
      self.gl.disable(glow::BLEND);
      self.gl.draw_elements(glow::LINE_LOOP, ebo_len.try_into().unwrap(), glow::UNSIGNED_INT, 0);
      
      // self.gl.disable(glow::PRIMITIVE_RESTART_FIXED_INDEX);
    }

    // TODO isomorphic double buffering across web/window mode
    // self.window.gl_swap_window();
  }
}

unsafe fn convert_f32_u8(data: &[f32]) -> &[u8] {
  core::slice::from_raw_parts(
    data.as_ptr() as *const u8,
    data.len() * 4, // 4 u8s per f32
  )
}

unsafe fn convert_u32_u8(data: &[u32]) -> &[u8] {
  core::slice::from_raw_parts(
    data.as_ptr() as *const u8,
    data.len() * 4 // 4 u8s per u32
  )
}