use std::error::Error;

//use std::fs;

use glow::*;
use std::collections::HashMap;

pub type UniformLocMap = HashMap<(String, GlslType), UniformLocation>;

pub struct Shader {
  // pub vert_path    : &'a str,
  // pub frag_path    : &'a str,
  pub program      : glow::Program,
  pub uniform_locs : UniformLocMap
}

// impl<'a> Drop for Shader<'a> {
//   fn drop(&mut self) {
//     unsafe {
//       self.gl.delete_program(self.program);
//     }
//   }
// }

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GlslType {
  Float,
  Vec3
}

/* --- static functions ----------------------------------------------------- */

impl Shader {

  // pub fn load(
  //   gl        : &glow::Context,
  //   vert_path : &'a str,
  //   frag_path : &'a str,
  //   uniforms  : &[(String, GlslType)]
  // ) -> Result<Shader<'a>, Box<dyn Error>>
  // {
  //   let program = Shader::program_from_paths(gl, vert_path, frag_path)?;
  //   let uniform_locs = Shader::get_uniform_locations(gl, program, uniforms.iter())?;

  //   return Ok(Shader {
  //     vert_path,
  //     frag_path,
  //     program,
  //     uniform_locs
  //   });
  // }

  pub fn build(
    gl: &glow::Context,
    vert_src : &str,
    frag_src : &str,
    uniforms  : &[(String, GlslType)]
  ) -> Result<Shader, Box<dyn Error>> {
    let program = unsafe {
      Shader::build_program(gl, vert_src, frag_src)
    }?;

    let uniform_locs = Shader::get_uniform_locations(gl, program, uniforms.iter())?;

    return Ok(Shader {
      program,
      uniform_locs
    });
  }

  pub fn get_uniform_locations<'a>(
    gl       : &glow::Context,
    program  : Program,
    uniforms : impl IntoIterator<Item=&'a (String, GlslType)>
  ) -> Result<
    HashMap<(String, GlslType), UniformLocation>,
    Box<dyn Error>
  > {
    let mut result = HashMap::new();

    for (name, typ) in uniforms {

      let loc;
      unsafe {
        loc = gl.get_uniform_location(program, name);
      }

      match loc {
        None => {
          return Err(format!("uniform {name} not found!").into());
        }
        Some(loc) => {
          result.insert((name.to_string(), *typ), loc);
        }
      }
    }

    return Ok(result);
  }

  // pub fn program_from_paths(
  //   gl        : &glow::Context,
  //   vert_path : &str,
  //   frag_path : &str
  // ) -> Result<glow::Program, Box<dyn Error>>
  // {
  //   let vert_src = fs::read_to_string(vert_path)?;
  //   let frag_src = fs::read_to_string(frag_path)?;

  //   unsafe {
  //     return Shader::build_program(gl, &vert_src, &frag_src);
  //   }
  // }
  
  pub unsafe fn build_program(
    gl       : &glow::Context,
    vert_src : &str,
    frag_src : &str
  ) -> Result<glow::Program, Box<dyn Error>>
  {
    // create shader program
    let program = gl.create_program()?;

    let shader_sources = [
      (glow::VERTEX_SHADER, vert_src),
      (glow::FRAGMENT_SHADER, frag_src),
    ];

    // compile vertex and fragment shaders
    let mut shaders = Vec::with_capacity(shader_sources.len());
    
    for &(shader_type, shader_source) in shader_sources.iter() {
      let shader = compile_shader(gl, shader_type, shader_source)?;
      gl.attach_shader(program, shader);
      shaders.push(shader);
    }

    // link shader program
    gl.link_program(program);
    if !gl.get_program_link_status(program) {
      let err = gl.get_program_info_log(program);
      return Err(format!("failed to link shader program! {err}").into());
    }

    // free shaders
    for shader in shaders {
      gl.detach_shader(program, shader);
      gl.delete_shader(shader);
    }

    return Ok(program);
  }
}

/* ---- methods ------------------------------------------------------------- */

impl Shader {

  // pub fn reload(
  //   &mut self,
  //   gl : &glow::Context,
  // ) -> Result<(), Box<dyn Error>> {
  //   Shader::program_from_paths(gl, self.vert_path, self.frag_path).and_then(|program| {
  //     Shader::get_uniform_locations(gl, program, self.uniform_locs.keys()).map(|uniform_locs| {
  //       // reload successful, delete old program
  //       unsafe {
  //         gl.delete_program(self.program);
  //       }

  //       // save new program
  //       self.program = program;
  //       self.uniform_locs = uniform_locs;
  //     })
  //   })
  // }

  pub fn activate(
    &self,
    gl : &glow::Context
  ) {
    unsafe {
      gl.use_program(Some(self.program));
    }
  }

}

/* ---- uniforms ------------------------------------------------------------ */

impl Shader {

  pub fn set_uniform_float(&mut self, gl: &glow::Context, name: &str, value: f32) {
    let loc = self.uniform_locs.get(&(name.to_string(), GlslType::Float));

    if loc.is_none() {
      eprintln!("cannot find uniform float {name}");
    }

    unsafe {
      gl.uniform_1_f32(loc, value);
    }
  }

  pub fn set_uniform_vec3(&mut self, gl: &glow::Context, name: &str, x: f32, y: f32, z: f32) {
    let loc = self.uniform_locs.get(&(name.to_string(), GlslType::Vec3));

    if loc.is_none() {
      eprintln!("cannot find uniform vec3 {name}");
    }

    unsafe {
      gl.uniform_3_f32(loc, x, y, z);
    }
  }

}

/* -------------------------------------------------------------------------- */

unsafe fn compile_shader(
  gl: &glow::Context,
  shader_type: u32,
  shader_source: &str
) -> Result<glow::Shader, Box<dyn Error>> {
  let shader = gl.create_shader(shader_type)?;
  gl.shader_source(shader, shader_source);
  gl.compile_shader(shader);
  if !gl.get_shader_compile_status(shader) {
    let err = gl.get_shader_info_log(shader);
    return Err(format!("failed to compile shader! {err}").into());
  }
  return Ok(shader);
}