use glow::HasContext;
use std::rc::Rc;

pub struct BatchPolyRenderer {
  gl: Rc<glow::Context>,
  vao: glow::VertexArray,
  vbo: glow::Buffer,
  ebo: glow::Buffer
}

impl Drop for BatchPolyRenderer {
  fn drop(&mut self) {
    unsafe {
      self.gl.delete_vertex_array(self.vao);
      self.gl.delete_buffer(self.vbo);
      self.gl.delete_buffer(self.ebo);
    }
  }
}

impl BatchPolyRenderer {
  pub fn build(
    gl: Rc<glow::Context>
  ) -> Self {
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

    BatchPolyRenderer {
      gl,
      vbo,
      vao,
      ebo
    }
  }

  pub fn render(
    &self,
    vbo_data: Vec<f32>,
    ebo_data: Vec<u32>,
    num_shapes: u32,
    max_vbo_idx: u32,
    show_fill: bool,
    show_outline: bool
  ) {
    let vbo_data_u8;
    let ebo_data_u8;
    unsafe {
      vbo_data_u8 = convert_f32_u8(&vbo_data);
      ebo_data_u8 = convert_u32_u8(&ebo_data);
    }

    unsafe {
      // binding the VAO automatically binds VBO/EBO
      self.gl.bind_vertex_array(Some(self.vao));

      // write new vertex buffer
      self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
      self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vbo_data_u8, glow::DYNAMIC_DRAW);

      // write new element buffer
      self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
      self.gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, ebo_data_u8, glow::DYNAMIC_DRAW);

      // single draw call
      // https://registry.khronos.org/webgl/specs/latest/2.0/#5.18
      // self.gl.enable(glow::PRIMITIVE_RESTART_FIXED_INDEX);
      let ebo_len = max_vbo_idx + num_shapes;
      
      if show_fill {
        self.gl.enable(glow::BLEND);
        self.gl.blend_func(glow::SRC_COLOR, glow::ONE_MINUS_SRC_COLOR);
        self.gl.draw_elements(glow::TRIANGLE_FAN, ebo_len.try_into().unwrap(), glow::UNSIGNED_INT, 0);
        self.gl.disable(glow::BLEND);
      }

      // outlines
      if show_outline {
        self.gl.draw_elements(glow::LINE_LOOP, ebo_len.try_into().unwrap(), glow::UNSIGNED_INT, 0);
      }

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