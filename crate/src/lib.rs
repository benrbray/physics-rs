mod utils;
mod webgl;
mod game;

use game::world::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use std::process;


// expose an async initThreadPool function in the final generated JavaScript for your library.
pub use wasm_bindgen_rayon::init_thread_pool;

////////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  // The `console.log` is quite polymorphic, so we can bind it with multiple
  // signatures. Note that we need to use `js_name` to ensure we always call
  // `log` in JS.
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  // Multiple arguments too!
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
  console_log!("Hello, wasm-physics!");
}

////////////////////////////////////////////////////////////////////////////////

// static mut GAME: Option<Game> = None;

#[wasm_bindgen]
pub fn init_game(canvas: HtmlCanvasElement) {
  // better wasm error messages
  utils::set_panic_hook();

  console_log!("init_game");

  let _ = webgl::core::init_webgl(canvas);

  // game
  let game = Game::build();
  // .unwrap_or_else(|_err| {
  //   eprintln!("failed to initialize game!");
  //   process::exit(1);
  // });
  
  // TODO: avoid unsafe mutable static
  // unsafe {
  //   GAME = Some(game::world::init_game().unwrap());
  // }
}

#[wasm_bindgen]
pub fn tick() {
  console_log!("tick!");
  // let game = unsafe { GAME.as_mut().unwrap() };
  // game.tick();
}