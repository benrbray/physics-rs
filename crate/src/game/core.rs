use std::rc::Rc;

use glow::Context;
use specs::prelude::*;

use crate::game::components;
use crate::geom;
use super::systems::time::{PrintTimeSystem,Time};
use super::systems::render::RenderSystem;

////////////////////////////////////////////////////////////////////////////////

pub struct Game<'a> {
  world: World,
  update_dispatcher: Dispatcher<'a, 'a>,
  render_dispatcher: Dispatcher<'a, 'a>
}

impl<'a> Game<'a> {
  pub fn new(
    gl: Rc<Context>
  ) -> Game<'a> {
    
    /* ---- update dispatcher ---- */

    let update_builder = DispatcherBuilder::new();
    let mut update_dispatcher = update_builder
      .with(PrintTimeSystem, "print_time", &[])
      .build();

    /* ---- render dispatcher ---- */

    let render_system = RenderSystem::build(&gl);
    
    let render_dispatcher = {
      let render_builder = DispatcherBuilder::new();
      
      render_builder
        // thread-local systems always execute at the end of dispatch
        .with_thread_local(render_system)
        .build()
    };

    // set up world
    let mut world = World::new();
    update_dispatcher.setup(&mut world);

    world.insert(Time(0.0));
    

    let mut game = Game {
      world,
      update_dispatcher,
      render_dispatcher
    };

    game.create_scene1();

    game
  }

  pub fn tick(&mut self) {
    {
      let mut sim_time = self.world.write_resource::<Time>();
      *sim_time = Time(sim_time.0 + 1.0);
    }

    self.update_dispatcher.dispatch(&self.world);
    self.world.maintain();
  }

  pub fn render(&mut self) {
    self.render_dispatcher.dispatch(&self.world);
    self.world.maintain();
  }
}

impl<'a> Game<'a> {
  pub fn create_scene1(&mut self) {
    for k in 0..100 {
      let n = 3 + (k % 6);
      let px = 2.0 * (0.1 * (k as f32));
      let py = 2.0 * (0.1 * (k as f32));
      let vx = 0.001;
      let vy = 0.001;

      self.world.create_entity()
        .with(components::Geom2d { shape : geom::ConvexPoly::regular(n, 0.08) })
        .with(components::Position { pos : (px, py) })
        .with(components::Velocity { x : vx, y : vy })
        .build();
    }
  }
}