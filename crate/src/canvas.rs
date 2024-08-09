// adapted from chinedufn/webgl-water-tutorial
// https://github.com/chinedufn/webgl-water-tutorial/blob/master/src/canvas.rs

use std::rc::Rc;

// use std::rc::Rc;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen::JsValue;
// use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use crate::{Game, Event};

////////////////////////////////////////////////////////////////////////////////

pub fn create_webgl_context(
  canvas: &HtmlCanvasElement,
) -> Result<WebGl2RenderingContext, JsValue> {
  let gl = canvas
    .get_context("webgl2")?
    .unwrap()
    .dyn_into::<WebGl2RenderingContext>()?;

  Ok(gl)
}

pub fn attach_events(
  canvas: &HtmlCanvasElement,
  game: Rc<Game<'static>>
) -> Result<(), JsValue> {
  attach_mouse_down_handler(canvas, Rc::clone(&game))?;

  Ok(())
}

fn attach_mouse_down_handler (
  canvas: &HtmlCanvasElement,
  game: Rc<Game<'static>>
) -> Result<(), JsValue> {
  let handler = move |event: web_sys::MouseEvent| {
    let x = event.client_x();
    let y = event.client_y();
    event.prevent_default();
    game.send_event(Event::MouseDown(x, y))
  };

  let handler = Closure::<dyn FnMut(_)>::new(handler);
  canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;

  // TODO (Ben @ 2024/08/09) will forget() leak memory?
  handler.forget();
  
  Ok(())
}