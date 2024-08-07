use specs::{prelude::*, storage::VecStorage};
use specs_derive::Component;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
  pub pos: (f32, f32)
}