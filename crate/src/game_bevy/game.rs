use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use bevy_ecs::{event::EventRegistry, prelude::*, schedule::ScheduleLabel};
use glow::{Context, HasContext};

use crate::game_bevy::{events::InputEvent, resources::game_state::{game_state_event_listener, GameState}, systems::{event_system::{event_writer_system, EventQueue, EventQueueResource}, physics_system, player_control_system::player_control_system, render_system::{render_system, RenderResource}}};

/* -------------------------------------------- */

pub struct Store {
  pub world: World,
  pub update_schedule: Schedule,
  pub render_schedule: Schedule,
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct Update;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct Render;

impl Store {
  pub fn tick(&mut self) {
    self.update_schedule.run(&mut self.world);
  }

  pub fn render(&mut self) {
    self.render_schedule.run(&mut self.world);
  }
}

/* -------------------------------------------- */

pub struct Game {
  pub state: Rc<RefCell<Store>>,
  pub events: Rc<RefCell<EventQueue>>
}

impl Game {
  pub fn new(
    gl: Rc<Context>
  ) -> Game {
    unsafe {
      gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }

    let mut world = World::new();

    /* ---- event registration ---- */

    EventRegistry::register_event::<InputEvent>(&mut world);

    /* ---- resources ---- */
    world.insert_non_send_resource(RenderResource::build(gl));

    let events = Rc::new(RefCell::new(EventQueue {
      events: VecDeque::new()
    }));

    world.insert_non_send_resource(EventQueueResource {
      event_queue: Rc::clone(&events)
    });

    world.insert_resource(GameState {
      key_down: false,
      key_left: false,
      key_right: false,
      key_up: false
    });

    /* ---- update schedule ---- */
    let mut update_schedule = Schedule::new(Update);
    // update_schedule.add_systems(
    //   event_writer_system.before(
    //     player_control_system
    //   ).before(
    //     physics_system
    //   ));
    update_schedule.add_systems(
      (event_writer_system,
        game_state_event_listener,
      player_control_system,
      physics_system)
    );

    /* ---- render schedule ---- */
    let mut render_schedule = Schedule::new(Render);
    render_schedule.add_systems(render_system);
    
    /* ---- game state ---- */
    let state = Rc::new(RefCell::new(Store {
      world,
      update_schedule,
      render_schedule,
    }));
    
    return Game {
      state,
      events
    };
  }

  pub fn tick(&self) {
    self.state.borrow_mut().tick();
  }

  pub fn render(&self) {
    self.state.borrow_mut().render();
  }

  /**
   * Add an event to the event queue.  It will be added to
   * the bevy_ecs event system at the start of the next frame.
   */
  pub fn send_event(&self, event: InputEvent) {
    self.events.borrow_mut().events.push_back(event);
  }
}