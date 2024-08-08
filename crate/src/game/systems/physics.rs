use specs::prelude::*;
use crate::game::components;

pub struct PhysicsSystem;

// resources required for execution
#[derive(SystemData)]
pub struct PhysicsSystemData<'a> {
  position: WriteStorage<'a, components::Position>,
  velocity: ReadStorage<'a, components::Velocity>
}

impl PhysicsSystem {
  pub fn build() -> PhysicsSystem {
    PhysicsSystem
  }
}

impl<'b> System<'b> for PhysicsSystem {
  type SystemData = PhysicsSystemData<'b>;

  fn run(&mut self, mut data: PhysicsSystemData) {
    for (pos, vel) in (&mut data.position, &data.velocity).join() {
      pos.pos.0 += vel.x;
      pos.pos.1 += vel.y;

      while pos.pos.0 < -1.0 { pos.pos.0 += 2.0; }
      while pos.pos.0 >  1.0 { pos.pos.0 -= 2.0; }
      while pos.pos.1 < -1.0 { pos.pos.1 += 2.0; }
      while pos.pos.1 >  1.0 { pos.pos.1 -= 2.0; }
    }
  }
}