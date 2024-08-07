use specs::prelude::*;
use crate::console::*;

#[derive(Default)]
pub struct Time(pub f32);

pub struct PrintTimeSystem;

impl<'a> System<'a> for PrintTimeSystem {
  type SystemData = Read<'a, Time>;

  fn run(&mut self, data: Self::SystemData) {
    let time = data;
    console_log!("time: {}", time.0);
  }
}