use bevy_ecs::prelude::*;

use crate::geom;
use crate::bvh::aabb::AABB;

#[derive(Component)]
pub struct Position { pub pos: (f32, f32) }

#[derive(Component)]
pub struct Velocity { pub x: f32, pub y: f32 }

#[derive(Component)]
pub struct Collider {
  pub volume: AABB
}

#[derive(Component)]
pub struct Geom2d {
  pub shape: geom::ConvexPoly
}

#[derive(Component)]
pub struct Player;