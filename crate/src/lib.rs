mod utils;
mod webgl;
mod game;
mod geom;
mod canvas;
mod console;

use console::*;
use game::*;
use glow::Context;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use std::{cell::RefCell, rc::Rc};

#[wasm_bindgen]
pub fn greet() {
  console_log!("Hello, wasm-physics!");
}

////////////////////////////////////////////////////////////////////////////////

/// Used to control the application from JavaScript
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebClient {
  game: RefCell<Game<'static>>
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebClient {
  /// Initialize the WebClient.
  #[wasm_bindgen(constructor)]
  pub fn new(
    canvas: HtmlCanvasElement
  ) -> WebClient {
    utils::set_panic_hook();


    // webgl
    // let ctx = canvas::create_webgl_context(canvas).unwrap();
    // let gl = glow::Context::from_webgl2_context(ctx);
    // let game = RefCell::new(Game::new(&gl));

    let game = RefCell::new(Game::new(canvas));

    WebClient {
      game
    }
  }

  /// Perform one step of simulation.
  #[wasm_bindgen]
  pub fn tick(&self) -> Result<(), JsValue> {
    let mut game = self.game.borrow_mut();
    game.tick();
    game.render();
    
    Ok(())
  }
}