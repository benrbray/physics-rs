use specs::prelude::*;
use crate::game::components;
use crate::bvh::aabb;

////////////////////////////////////////////////////////////////////////////////

pub struct BroadPhaseData {
  bvh: aabb::Tree<()> 
}

impl Default for BroadPhaseData {
  fn default() -> BroadPhaseData {
    BroadPhaseData {
      bvh : aabb::Tree::new()
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CollisionSystem;

// resources required for execution
#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
  broad_phase     : Read<'a, BroadPhaseData>,
  collision_pairs : WriteStorage<'a, components::CollisionPair>
}

impl CollisionSystem {
  pub fn build() -> CollisionSystem {
    CollisionSystem
  }

  pub fn register(&self, ) {
    
  }
}

impl<'b> System<'b> for CollisionSystem {
  type SystemData = CollisionSystemData<'b>;

  fn run(&mut self, mut data: CollisionSystemData) {
    
  }
}