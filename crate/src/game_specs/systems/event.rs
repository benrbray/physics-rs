use specs::prelude::*;
use crate::game_specs::event::Event;
use crate::game_specs::state;
use crate::console::*;
use crate::game_specs::controls::keyboard::Key;

pub struct EventSystem;

// resources required for execution
#[derive(SystemData)]
pub struct EventSystemData<'a> {
	game_state: Write<'a, state::GameState>,
	event_queue: Write<'a, state::EventQueue>
}

impl EventSystem {
  pub fn build() -> EventSystem {
    EventSystem
  }
}

impl<'b> System<'b> for EventSystem {
  type SystemData = EventSystemData<'b>;

  fn run(&mut self, mut data: EventSystemData) {
    while let Some(cmd) = data.event_queue.events.pop_front() {
      match cmd {
        Event::MouseDown(x,y) => {
          // console_log!("mouse_down");
        }
        Event::KeyDown(key) => {
          // console_log!("key_down: {:#?}", key);
          match key {
            Key::ArrowDown  => { data.game_state.key_down  = true; }
            Key::ArrowUp    => { data.game_state.key_up    = true; }
            Key::ArrowLeft  => { data.game_state.key_left  = true; }
            Key::ArrowRight => { data.game_state.key_right = true; }
            _ => { }
          };
        }
        Event::KeyUp(key) => {
          // console_log!("key_up: {:#?}", key);
          match key {
            Key::ArrowDown  => { data.game_state.key_down  = false; }
            Key::ArrowUp    => { data.game_state.key_up    = false; }
            Key::ArrowLeft  => { data.game_state.key_left  = false; }
            Key::ArrowRight => { data.game_state.key_right = false; }
            _ => { }
          };
        }
      }
    }
  }
}