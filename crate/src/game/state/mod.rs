
use std::collections::VecDeque;

use super::event::Event;

#[derive(Debug, Default)]
pub struct GameState {
  pub key_left: bool,
  pub key_right: bool,
  pub key_up: bool,
  pub key_down: bool
}

#[derive(Default)]

pub struct EventQueue {
  pub events: VecDeque<Event>
}