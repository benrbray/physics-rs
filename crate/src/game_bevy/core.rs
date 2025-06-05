use bevy_ecs::prelude::*;

use crate::game_bevy::components::{Position, Velocity};

// // This system moves each entity with a Position and Velocity component
// fn movement(mut query: Query<(&mut Position, &Velocity)>) {
//   for (mut position, velocity) in &mut query {
//     position.x += velocity.x;
//     position.y += velocity.y;
//   }
// }

// fn main() {
//   // Create a new empty World to hold our Entities and Components
//   let mut world = World::new();

//   // Spawn an entity with Position and Velocity components
//   world.spawn((
//       Position { x: 0.0, y: 0.0 },
//       Velocity { x: 1.0, y: 0.0 },
//   ));

//   // Create a new Schedule, which defines an execution strategy for Systems
//   let mut schedule = Schedule::default();

//   // Add our system to the schedule
//   schedule.add_systems(movement);

//   // Run the schedule once. If your app has a "loop", you would run this once per loop
//   schedule.run(&mut world);
// }