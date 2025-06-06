mod utils;
mod webgl;
mod game_specs;
mod game_bevy;
mod geom;
mod graphics;
mod canvas;
mod console;
mod controls;
mod bvh;

use console::*;
use game_bevy::*;
use glow::{Context, HasContext};
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
  game: Rc<Game>
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebClient {
  /// Initialize the WebClient.
  #[wasm_bindgen(constructor)]
  pub fn new(
    canvas: HtmlCanvasElement
  ) -> Result<WebClient, JsValue> {
    use crate::game_bevy::scenes::scene1::create_scene1;

    utils::set_panic_hook();


    // webgl
    canvas.set_width(600);
    canvas.set_height(600);
    let ctx = canvas::create_webgl_context(&canvas).unwrap();
    let gl = glow::Context::from_webgl2_context(ctx);

    let game = Rc::new(Game::new(Rc::new(gl)));
    create_scene1(game.as_ref());

    canvas::attach_events(&canvas, Rc::clone(&game))?;

    Ok(WebClient {
      game
    })
  }

  /// Perform one step of simulation.
  #[wasm_bindgen]
  pub fn tick(&mut self) -> Result<(), JsValue> {
    // let game = Rc::make_mut(&mut self.game).unwrap();
    let game = &self.game;
    game.tick();
    game.render();
    
    Ok(())
  }
}