use crate::controls::keyboard::Key;
use bevy_ecs::prelude::*;

pub enum InputKind {
  MouseDown(i32, i32),
  KeyDown(Key),
  KeyUp(Key)
}

#[derive(Event)]
pub struct InputEvent {
  pub kind: InputKind
}