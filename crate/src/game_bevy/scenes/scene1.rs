use rand::prelude::*;

use crate::geom;
use crate::bvh::aabb::AABB;
use crate::game_bevy::game;
use crate::game_bevy::components;

pub fn create_scene1(game: &game::Game) {
  let mut state = game.state.borrow_mut();

  // spawn player
  state.world.spawn((
    components::Player,
    components::Geom2d { shape : geom::ConvexPoly::regular(3, 0.04) },
    components::Collider { volume : AABB {
      lower_bound : nalgebra::Vector2::new(-0.05, 0.05),
      upper_bound : nalgebra::Vector2::new(-0.05, 0.05)
    } },
    components::Position { pos : (0.0, 0.0) },
    components::Velocity { x : 0.0, y : 0.0 }
  ));
  
  // spawn asteroids
  for _ in 0..20 {
    let n = 3 + rand::thread_rng().gen_range(0, 6);
    let px = 2.0 * (rand::random::<f32>() * 2.0 - 1.0);
    let py = 2.0 * (rand::random::<f32>() * 2.0 - 1.0);
    let vx = 0.001 * (rand::random::<f32>() * 2.0 - 1.0);
    let vy = 0.001 * (rand::random::<f32>() * 2.0 - 1.0);

    let shape = geom::ConvexPoly::regular(n, 0.08);
    
    let min_x = shape.points.row(0).min();
    let max_x = shape.points.row(0).max();
    let min_y = shape.points.row(1).min();
    let max_y = shape.points.row(1).max();

    let aabb  = AABB {
      lower_bound : nalgebra::Vector2::new(min_x, min_y),
      upper_bound : nalgebra::Vector2::new(max_x, max_y)
    };

    state.world.spawn((
      components::Geom2d { shape },
      components::Position { pos : (px, py) },
      components::Velocity { x : vx, y : vy },
      components::Collider { volume : aabb },
    ));

  }
}