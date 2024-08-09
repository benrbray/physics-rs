use std::rc::Rc;

use specs::prelude::*;
use crate::game::components;
use crate::{state, Event, Game};
use crate::console::*;
use crate::game::controls::keyboard::Key;

pub struct EventSystem {
  game : Rc<Game<'static>>
}

// resources required for execution
#[derive(SystemData)]
pub struct EventSystemData<'a> {
	game_state: Write<'a, state::GameState>
}

impl EventSystem {
  pub fn build(
    game: Rc<Game<'static>>
  ) -> EventSystem {
    EventSystem {
      game
    }
  }
}

impl<'b> System<'b> for EventSystem {
  type SystemData = EventSystemData<'b>;

  fn run(&mut self, mut data: EventSystemData) {
    let mut store = self.game.store.borrow_mut();

    while let Some(cmd) = store.events.pop_front() {
      match cmd {
        Event::MouseDown(x,y) => {
          console_log!("mouse_down");
        }
        Event::KeyDown(key) => {
          console_log!("key_down: {:#?}", key);
          match key {
            Key::ArrowDown  => { data.game_state.key_down  = true; }
            Key::ArrowUp    => { data.game_state.key_up    = true; }
            Key::ArrowLeft  => { data.game_state.key_left  = true; }
            Key::ArrowRight => { data.game_state.key_right = true; }
            _ => { }
          };
        }
        Event::KeyUp(key) => {
          console_log!("key_up: {:#?}", key);
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