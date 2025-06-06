use bevy_ecs::prelude::*;
use crate::{controls::keyboard::Key, game_bevy::events};
use crate::console::*;

#[derive(Resource)]
pub struct GameState {
  pub key_left: bool,
  pub key_right: bool,
  pub key_up: bool,
  pub key_down: bool
}

pub fn game_state_event_listener(
  mut game_state: ResMut<GameState>,
  mut event_reader: EventReader<events::InputEvent>
) {
  for input_event in event_reader.read() {
    match &input_event.kind {
      events::InputKind::MouseDown(x,y) => {
        console_log!("mouse_down");
      }
      events::InputKind::KeyDown(key) => {
        console_log!("key_down: {:#?}", key);
        match key {
          Key::ArrowDown  => { game_state.key_down  = true; }
          Key::ArrowUp    => { game_state.key_up    = true; }
          Key::ArrowLeft  => { game_state.key_left  = true; }
          Key::ArrowRight => { game_state.key_right = true; }
          _ => { }
        };
      }
      events::InputKind::KeyUp(key) => {
        console_log!("key_up: {:#?}", key);
        match key {
          Key::ArrowDown  => { game_state.key_down  = false; }
          Key::ArrowUp    => { game_state.key_up    = false; }
          Key::ArrowLeft  => { game_state.key_left  = false; }
          Key::ArrowRight => { game_state.key_right = false; }
          _ => { }
        };
      }
    }
  }
}