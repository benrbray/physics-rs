// use web_sys::{WebGl2RenderingContext, WebGlShader };

// pub fn compile_shader(
//     context: &WebGl2RenderingContext,
//     shader_type: u32,
//     source: &str,
// ) -> Result<WebGlShader, String> {
//     let shader = context
//         .create_shader(shader_type)
//         .ok_or_else(|| String::from("Unable to create shader object"))?;
//     context.shader_source(&shader, source);
//     context.compile_shader(&shader);

//     if context
//         .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         Ok(shader)
//     } else {
//         Err(context
//             .get_shader_info_log(&shader)
//             .unwrap_or_else(|| String::from("Unknown error creating shader")))
//     }
// }