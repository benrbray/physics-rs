use bevy_ecs::prelude::*;
use crate::game_bevy::components;

/* ---------------------------------------- */

type PhysicsSystemData<'a> = (
  &'a mut components::Position,
  &'a     components::Velocity
);

// renderer must always run on main thread
// so we use bevy's NonSend
// https://bevy-cheatbook.github.io/programming/non-send.html
pub fn physics_system(
  data: Query<PhysicsSystemData>
) {
  for (mut pos, vel) in data {
    pos.pos.0 += vel.x;
    pos.pos.1 += vel.y;

    while pos.pos.0 < -1.0 { pos.pos.0 += 2.0; }
    while pos.pos.0 >  1.0 { pos.pos.0 -= 2.0; }
    while pos.pos.1 < -1.0 { pos.pos.1 += 2.0; }
    while pos.pos.1 >  1.0 { pos.pos.1 -= 2.0; }
  }
}