use specs::prelude::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct Time(f32);

struct PrintTimeSystem;

impl<'a> System<'a> for PrintTimeSystem {
    type SystemData = Read<'a, Time>;

    fn run(&mut self, data: Self::SystemData) {
        let time = data;
				println!("time: {}", time.0);
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Game<'a> {
  // ecs
  pub world: World,
  pub update_dispatcher: Dispatcher<'a, 'a>
}

impl<'a> Game<'a> {
  pub fn build() -> Result<Game<'a>, ()> {
    // ecs dispatcher
    let update_builder = DispatcherBuilder::new();

    let mut update_dispatcher = update_builder
      .with(PrintTimeSystem, "print_time", &[])
      .build();

    // world
    let mut world = World::new();
    update_dispatcher.setup(&mut world);

    // time
    world.insert(Time(0.0));

    Ok(Game {
      world,
      update_dispatcher
    })
  }
}

pub struct GameWasm {
  game: Game<'static>
}

pub fn build_game() -> Result<GameWasm, ()> {
  let game = Game::build().unwrap();
  return Ok(GameWasm { game });
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> Game<'a> {
  pub fn tick(&mut self) {
		{
			let mut sim_time = self.world.write_resource::<Time>();
			*sim_time = Time(sim_time.0 + 1.0);
		}

    self.update_dispatcher.dispatch(&self.world);
    self.world.maintain();
  }
}