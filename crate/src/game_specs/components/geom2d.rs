use specs::{prelude::*, storage::VecStorage};
use specs_derive::Component;
use crate::geom;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Geom2d {
  pub shape: geom::ConvexPoly
}