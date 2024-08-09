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

impl<'b> System<'b> for PlayerControlSystem {
  type SystemData = PlayerControlSystemData<'b>;

  fn run(&mut self, mut data: PlayerControlSystemData) {
		for (vel, _) in (&mut data.velocity, &data.player).join() {
			const SPEED: f32 = 0.2;
			if data.game_state.key_up    { vel.y -= SPEED; }
			if data.game_state.key_down  { vel.y += SPEED; }
			if data.game_state.key_left  { vel.x -= SPEED; }
			if data.game_state.key_right { vel.x += SPEED; }
		}
	}
}