mod utils;
mod webgl;
mod game;
mod canvas;
mod console;

use console::*;
use game::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use std::{cell::RefCell, rc::Rc};

#[wasm_bindgen]
pub fn greet() {
  console_log!("Hello, wasm-physics!");
}

////////////////////////////////////////////////////////////////////////////////

// used to control the application from JavaScript
#[wasm_bindgen]
pub struct WebClient {
  game: RefCell<Game>,
  gl: Rc<WebGl2RenderingContext>
}
#[wasm_bindgen]
impl WebClient {
  /// Initialize the WebClient.
  #[wasm_bindgen(constructor)]
  pub fn new(
    canvas: HtmlCanvasElement
  ) -> WebClient {
    utils::set_panic_hook();

    let game = RefCell::new(Game::new());
    let gl = Rc::new(canvas::create_webgl_context(canvas).unwrap());

    WebClient { game, gl }
  }

  /// Start simulation and rendering.
  #[wasm_bindgen]
  pub fn start(&self) -> Result<(), JsValue> {
    Ok(())
  }

  #[wasm_bindgen]
  pub fn tick(&self) -> Result<(), JsValue> {
    self.game.borrow_mut().tick();
    Ok(())
  }
}

////////////////////////////////////////////////////////////////////////////////

// static mut GAME: Option<Game> = None;

// #[wasm_bindgen]
// pub fn init_game(canvas: HtmlCanvasElement) {
//   // better wasm error messages
//   utils::set_panic_hook();

//   console_log!("init_game");

//   let _ = WebClient::new(canvas);

//   // let _ = webgl::core::init_webgl(canvas);

//   // game
//   // let game = Game::build();
//   // .unwrap_or_else(|_err| {
//   //   eprintln!("failed to initialize game!");
//   //   process::exit(1);
//   // });
  
//   // TODO: avoid unsafe mutable static
//   // unsafe {
//   //   GAME = Some(game::world::init_game().unwrap());
//   // }
// }

// #[wasm_bindgen]
// pub fn tick() {
//   console_log!("tick!");
//   // let game = unsafe { GAME.as_mut().unwrap() };
//   // game.tick();
// }