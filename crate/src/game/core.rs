use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use glow::{Context, HasContext};
use specs::prelude::*;
use web_sys::HtmlCanvasElement;
use rand::prelude::*;

use crate::console::*;
use crate::game::components;
use crate::canvas;
use crate::geom;
use crate::state;
use crate::graphics::shader::Shader;
use crate::state::EventQueue;
use super::event::Event;
use super::systems::player_control::PlayerControlSystem;
use super::systems::time::Time;
use super::systems::render::RenderSystem;
use super::systems::physics::PhysicsSystem;
use super::systems::event::EventSystem;

////////////////////////////////////////////////////////////////////////////////

pub enum Command {
  // graphics
  ReloadShader,
  // application
  Pause,
  Quit
}

/// Holds all mutable state related to the `Game`.
pub struct Store<'a> {
  // game loop
  pub events   : VecDeque<Event>,
  pub commands : VecDeque<Command>,
  // specs
  pub world: World,
  pub update_dispatcher: Dispatcher<'a, 'a>,
  pub render_dispatcher: Dispatcher<'a, 'a>,
}

pub struct Game<'a> {
  // game loop
  pub store   : Rc<RefCell<Store<'a>>>,
  // graphics
  gl: Rc<Context>,
  pub shader : Shader
}

impl<'a> Game<'a> {
  pub fn new(
    canvas: &HtmlCanvasElement
    // gl: &'a Context
  ) -> Game<'a> {
    canvas.set_width(600);
    canvas.set_height(600);
    
    /* ---- update dispatcher ---- */

    let update_builder = DispatcherBuilder::new();
    let mut update_dispatcher = update_builder
      // .with(PrintTimeSystem, "print_time", &[])
	    .with(EventSystem::build(), "event_system", &[])
	    .with(PlayerControlSystem::build(), "player_control", &[])
	    .with(PhysicsSystem::build(), "physics", &[])
      .build();

    /* ---- render dispatcher ---- */

    let ctx = canvas::create_webgl_context(canvas).unwrap();
    let gl = Rc::new(glow::Context::from_webgl2_context(ctx));
    unsafe {
      gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }
    
    let mut render_dispatcher = {
      let render_system = RenderSystem::build(Rc::clone(&gl));
      let render_builder = DispatcherBuilder::new();
      
      render_builder
        // thread-local systems always execute at the end of dispatch
        .with_thread_local(render_system)
        .with_barrier()
        .build()
    };

    /* ---- world ---- */

    let mut world = World::new();
    update_dispatcher.setup(&mut world);
    render_dispatcher.setup(&mut world);

    world.insert(Time(0.0));

    world.insert(state::GameState {
      key_left: false,
      key_right: false,
      key_up: false,
      key_down: false
    });

    world.create_entity()
      .with(components::Geom2d { shape : geom::ConvexPoly::regular(3, 0.04) })
      .with(components::Player)
      .with(components::Position { pos : (0.0, 0.0) })
      .with(components::Velocity { x : 0.0, y : 0.0 })
      .build();

    /* ---- compile shaders ---- */

    let vert_src = include_str!("../../shaders/basic/basic.vert");
    let frag_src = include_str!("../../shaders/basic/basic.frag");
    let shader = Shader::build(&gl, vert_src, frag_src, &[]).unwrap();

    shader.activate(&gl);
    
    /* -------- */

    let store = Rc::new(RefCell::new(Store {
      events : VecDeque::new(),
      commands : VecDeque::new(),
      world,
      update_dispatcher,
      render_dispatcher,
    }));
    
    let mut game = Game {
      store,
      gl,
      shader
    };

    game.create_scene1();
    game.store.borrow_mut().world.maintain();

    game
  }
}

/* ---- Update -------------------------------------------------------------- */

impl<'a> Store<'a> {
  pub fn update(&mut self) {
    self.process_commands();
    self.process_events();

    {
      // timekeeping
      let mut sim_time = self.world.write_resource::<Time>();
      *sim_time = Time(sim_time.0 + 1.0);
    }

    self.update_dispatcher.dispatch(&self.world);
    self.world.maintain();
  }

  fn process_events(&mut self) {
    // TODO (Ben @ 2024/08/10) avoid transferring events between queues?
    while let Some(evt) = self.events.pop_front() {
      let mut queue = self.world.write_resource::<EventQueue>();
      queue.events.push_back(evt);
    }
  }

  fn process_commands(&mut self) -> bool {
    while let Some(cmd) = self.commands.pop_front() {
      match cmd {
        Command::Quit         => { return true; }
        Command::Pause        => {
          console_log!("pause");
        }
        Command::ReloadShader => {
          console_log!("reload shader");
          // self.shader.reload(self.gl).unwrap();
          // self.shader.activate(self.gl);
        }
      }
    }

    false
  }
}

impl<'a> Game<'a> {
  pub fn tick(&self) {
    let mut store = self.store.borrow_mut();
    store.update();
  }

  pub fn send_event(&self, event: Event) {
    self.store.borrow_mut().events.push_back(event);
  }
}

/* ---- Render -------------------------------------------------------------- */

impl<'a> Store<'a> {
  pub fn render(&mut self) {
    self.render_dispatcher.dispatch(&self.world);
  }
}

impl<'a> Game<'a> {
  pub fn render(&self) {
    self.store.borrow_mut().render();
  }
}

/* ---- Scene --------------------------------------------------------------- */

impl<'a> Game<'a> {
  pub fn create_scene1(&mut self) {
    let mut store = self.store.borrow_mut();
    for _ in 0..100 {
      let n = 3 + rand::thread_rng().gen_range(0, 6);
      let px = 2.0 * (rand::random::<f32>() * 2.0 - 1.0);
      let py = 2.0 * (rand::random::<f32>() * 2.0 - 1.0);
      let vx = 0.001 * (rand::random::<f32>() * 2.0 - 1.0);
      let vy = 0.001 * (rand::random::<f32>() * 2.0 - 1.0);

      store.world.create_entity()
        .with(components::Geom2d { shape : geom::ConvexPoly::regular(n, 0.08) })
        .with(components::Position { pos : (px, py) })
        .with(components::Velocity { x : vx, y : vy })
        .build();
    }
  }
}