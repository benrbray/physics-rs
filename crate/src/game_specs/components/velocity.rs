use specs::{prelude::*, storage::VecStorage};
use specs_derive::Component;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity {
  pub x: f32,
  pub y: f32
}