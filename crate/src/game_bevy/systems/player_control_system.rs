use bevy_ecs::prelude::*;
use crate::game_bevy::components;
use crate::game_bevy::resources::game_state::GameState;

pub fn player_control_system(
  game_state: Res<GameState>,
  query: Query<(&components::Player, &mut components::Velocity)>
) {
  for (_player, mut vel) in query {
    const FRICTION: f32 = 0.98;
    const ACCEL: f32 = 0.05;

    // apply friction
    vel.x *= FRICTION;
    vel.y *= FRICTION;
    // apply control
    if game_state.key_up    { vel.y = ACCEL;  }
    if game_state.key_down  { vel.y = -ACCEL; }
    if game_state.key_left  { vel.x = -ACCEL; }
    if game_state.key_right { vel.x = ACCEL;  }
  }
}