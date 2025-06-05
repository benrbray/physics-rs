use std::{cell::RefCell, rc::Rc};

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use glow::{Context, HasContext};
use web_sys::HtmlCanvasElement;

use crate::{canvas, game_bevy::systems::{physics_system, render_system::{render_system, RenderResource}}};

/* -------------------------------------------- */

pub struct GameState {
  pub world: World,
  pub update_schedule: Schedule,
  pub render_schedule: Schedule,
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct Update;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct Render;

impl GameState {
  pub fn tick(&mut self) {
    self.update_schedule.run(&mut self.world);
  }

  pub fn render(&mut self) {
    self.render_schedule.run(&mut self.world);
  }
}

/* -------------------------------------------- */

pub struct Game {
  pub state: Rc<RefCell<GameState>>,
}

impl Game {
  pub fn new(
    canvas: &HtmlCanvasElement
  ) -> Game {

    canvas.set_width(600);
    canvas.set_height(600);
    let ctx = canvas::create_webgl_context(&canvas).unwrap();
    let gl = Rc::new(glow::Context::from_webgl2_context(ctx));
    unsafe {
      gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }

    let mut world = World::new();

    /* ---- resources ---- */
    world.insert_non_send_resource(RenderResource::build(gl));

    /* ---- update schedule ---- */
    let mut update_schedule = Schedule::new(Update);
    update_schedule.add_systems(physics_system);

    /* ---- render schedule ---- */
    let mut render_schedule = Schedule::new(Render);
    render_schedule.add_systems(render_system);

    /* ---- game state ---- */
    let state = Rc::new(RefCell::new(GameState {
      world,
      update_schedule,
      render_schedule,
    }));

    return Game {
      state
    };
  }

  pub fn tick(&self) {
    
    self.state.borrow_mut().tick();
  }

  pub fn render(&self) {
    self.state.borrow_mut().render();
  }
}