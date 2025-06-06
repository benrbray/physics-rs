// adapted from chinedufn/webgl-water-tutorial
// https://github.com/chinedufn/webgl-water-tutorial/blob/master/src/canvas.rs

use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys;

use crate::game_bevy::Game;
use crate::game_bevy::events::{InputEvent, InputKind};
use crate::controls::keyboard;

////////////////////////////////////////////////////////////////////////////////

pub fn create_webgl_context(
  canvas: &web_sys::HtmlCanvasElement,
) -> Result<web_sys::WebGl2RenderingContext, JsValue> {
  let gl = canvas
    .get_context("webgl2")?
    .unwrap()
    .dyn_into::<web_sys::WebGl2RenderingContext>()?;

  Ok(gl)
}

pub fn attach_events(
  canvas: &web_sys::HtmlCanvasElement,
  game: Rc<Game>
) -> Result<(), JsValue> {
  attach_mouse_down_handler(canvas, Rc::clone(&game))?;
  attach_key_down_handler(canvas, Rc::clone(&game))?;
  attach_key_up_handler(canvas, Rc::clone(&game))?;

  Ok(())
}

fn attach_mouse_down_handler (
  canvas: &web_sys::HtmlCanvasElement,
  game: Rc<Game>
) -> Result<(), JsValue> {
  let handler = move |event: web_sys::MouseEvent| {
    let x = event.client_x();
    let y = event.client_y();
    event.prevent_default();
    game.send_event(InputEvent { kind: InputKind::MouseDown(x, y) });
  };

  let handler = Closure::<dyn FnMut(_)>::new(handler);
  canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;

  // TODO (Ben @ 2024/08/09) will forget() leak memory?
  handler.forget();
  
  Ok(())
}

fn attach_key_down_handler (
  canvas: &web_sys::HtmlCanvasElement,
  game: Rc<Game>
) -> Result<(), JsValue> {
  let handler = move |event: web_sys::KeyboardEvent| {
    if let Some(key) = keyboard::convert_key(event.key()) {
      game.send_event(InputEvent { kind: InputKind::KeyDown(key) });
      event.prevent_default();
    }
  };

  let handler = Closure::<dyn FnMut(_)>::new(handler);
  canvas.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())?;

  // TODO (Ben @ 2024/08/09) will forget() leak memory?
  handler.forget();
  
  Ok(())
}

fn attach_key_up_handler (
  canvas: &web_sys::HtmlCanvasElement,
  game: Rc<Game>
) -> Result<(), JsValue> {
  let handler = move |event: web_sys::KeyboardEvent| {
    if let Some(key) = keyboard::convert_key(event.key()) {
      game.send_event(InputEvent { kind: InputKind::KeyUp(key) });
      event.prevent_default();
    }
  };

  let handler = Closure::<dyn FnMut(_)>::new(handler);
  canvas.add_event_listener_with_callback("keyup", handler.as_ref().unchecked_ref())?;

  // TODO (Ben @ 2024/08/09) will forget() leak memory?
  handler.forget();
  
  Ok(())
}