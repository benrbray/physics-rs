// use std::rc::Rc;
// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub fn create_webgl_context(
  canvas: HtmlCanvasElement
) -> Result<WebGl2RenderingContext, JsValue> {
  let gl = canvas
    .get_context("webgl2")?
    .unwrap()
    .dyn_into::<WebGl2RenderingContext>()?;

  gl.clear_color(0.0, 0.0, 0.0, 1.0);
  gl.enable(GL::DEPTH_TEST);

  Ok(gl)
}