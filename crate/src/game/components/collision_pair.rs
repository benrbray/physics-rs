use specs::{prelude::*, storage::VecStorage};
use specs_derive::Component;

#[derive(Component)]
#[storage(VecStorage)]
pub struct CollisionPair {
  
}