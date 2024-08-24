use specs::prelude::*;
use crate::game::components;
use crate::state;

pub struct PlayerControlSystem;

// resources required for execution
#[derive(SystemData)]
pub struct PlayerControlSystemData<'a> {
  game_state: Read<'a, state::GameState>,
  player: ReadStorage<'a, components::Player>,
  velocity: WriteStorage<'a, components::Velocity>
}

impl PlayerControlSystem {
  pub fn build() -> PlayerControlSystem {
    PlayerControlSystem
  }
}

fn clamp_vel(v: f32) -> f32 {
  const MIN: f32 = 0.0001;
  const MAX: f32 = 0.05;

  let sign = v.signum();
  let abs  = v.abs();

  if abs < MIN { return 0.0;        }
  if abs > MAX { return sign * MAX; }
  
  return v;
}

impl<'b> System<'b> for PlayerControlSystem {
  type SystemData = PlayerControlSystemData<'b>;

  fn run(&mut self, mut data: PlayerControlSystemData) {
    for (vel, _) in (&mut data.velocity, &data.player).join() {
      const FRICTION: f32 = 0.98;
      const ACCEL: f32 = 0.05;

      // apply friction
      vel.x *= FRICTION;
      vel.y *= FRICTION;
      // apply control
      if data.game_state.key_up    { vel.y = ACCEL;  vel.x = 0.0; }
      if data.game_state.key_down  { vel.y = -ACCEL; vel.x = 0.0; }
      if data.game_state.key_left  { vel.x = -ACCEL; vel.y = 0.0; }
      if data.game_state.key_right { vel.x = ACCEL;  vel.y = 0.0; }

      // vel.x = clamp_vel(vel.x);
      // vel.y = clamp_vel(vel.y);
    }
  }
}