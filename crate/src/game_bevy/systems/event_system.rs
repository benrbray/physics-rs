use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use bevy_ecs::prelude::*;

use crate::game_bevy::events::InputEvent;

pub struct EventQueue {
  pub events : VecDeque<InputEvent>,
}

pub struct EventQueueResource {
  pub event_queue: Rc<RefCell<EventQueue>>
}

pub fn event_writer_system(
  mut event_writer: EventWriter<InputEvent>,
  event_queue: NonSendMut<EventQueueResource>
) {
  while let Some(event) = event_queue.event_queue.borrow_mut().events.pop_front() {
    event_writer.write(event);
  }
}