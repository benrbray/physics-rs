use specs::{prelude::*, storage::VecStorage};
use specs_derive::Component;

use crate::bvh::aabb::AABB;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Collider {
  pub volume: AABB
}