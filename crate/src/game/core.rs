use specs::prelude::*;
use super::systems::time::{PrintTimeSystem,Time};

pub struct Game {
  world: World,
  dispatcher: Dispatcher<'static, 'static>
}

impl Game {
  pub fn new() -> Game {
    let update_builder = DispatcherBuilder::new();
    let mut update_dispatcher = update_builder
      .with(PrintTimeSystem, "print_time", &[])
      .build();

    // set up world
    let mut world = World::new();
    update_dispatcher.setup(&mut world);

    world.insert(Time(0.0));
    
    Game {
      world,
      dispatcher: update_dispatcher
    }
  }

  pub fn tick(&mut self) {
    {
      let mut sim_time = self.world.write_resource::<Time>();
      *sim_time = Time(sim_time.0 + 1.0);
    }

    self.dispatcher.dispatch(&self.world);
    self.world.maintain();
  }
}