use nalgebra::{self, Vector2};
use slotmap::{new_key_type, SlotMap};
use std::collections::BinaryHeap;

////////////////////////////////////////////////////////////////////////////////

pub struct AABB {
  /// lower bound in each axis
  pub lower_bound: nalgebra::Vector2<f32>,
  /// upper bound in each axis
  pub upper_bound: nalgebra::Vector2<f32>
}

impl AABB {
  // TODO (Ben @ 2024/08/20) make efficient with SIMD
  pub fn join(a: &AABB, b: &AABB) -> AABB { 
    let lower_bound = nalgebra::Vector2::inf(&a.lower_bound, &b.lower_bound);
    let upper_bound = nalgebra::Vector2::inf(&a.upper_bound, &b.upper_bound);
    return AABB { lower_bound, upper_bound };
  }

  pub fn surface_area(&self) -> f32 {
    let diffs = self.upper_bound - self.lower_bound;
    return 2.0 * (diffs.x + diffs.y);
  }

  pub fn ray_cast(&self, p1: Vector2<f32>, p2: Vector2<f32>) -> bool {
    // TODO (Ben @ 2024/08/25) ray cast volumes
    panic!("not implemented");
  } 
}

////////////////////////////////////////////////////////////////////////////////

pub enum NodeKind<D> {
  Internal {
    child1: NodeIdx,
    child2: NodeIdx,
  },
  Leaf {
    data: D,
  }
}

// indexing type for slotmap
new_key_type! {
  struct NodeIdx;
}

pub struct Node<D> {
  parent : Option<NodeIdx>,
  volume : AABB,
  kind   : NodeKind<D>,
}

pub struct Tree<D> {
  nodes    : SlotMap<NodeIdx, Node<D>>,
  root_idx : Option<NodeIdx>
}

////////////////////////////////////////////////////////////////////////////////

impl<D> Tree<D> {
  pub fn new() -> Self {
    return Tree {
      nodes : SlotMap::default(),
      root_idx : None
    }
  }

  pub fn ray_cast(&self, p1: Vector2<f32>, p2: Vector2<f32>) -> bool {
    match self.root_idx {
      None => { return false; }
      Some(root_idx) => {
        // list of boxes to check for collisions
        let mut stack = vec![root_idx];

        while let Some(top_idx) = stack.pop() {
          if let Some(node) = self.nodes.get(top_idx) {
            // exit early if ray does not intersect this level
            if !node.volume.ray_cast(p1, p2) { continue; }

            match node.kind {
              // perform narrow phase for leaf nodes
              NodeKind::Leaf { .. } => {
                return true;
              }
              // ray cast children of internal nodes
              NodeKind::Internal { child1, child2 } => {
                stack.push(child1);
                stack.push(child2);
              }
            }
          } else {
            panic!("invalid index");
          }
        }

        return false;
      }
    }
  }

  /// dynamic insertion
  pub fn insert_leaf(&mut self, volume: AABB, data: D) {
    // create new leaf
    let leaf_idx: NodeIdx = self.nodes.insert(Node {
      volume,
      parent : None,
      kind   : NodeKind::Leaf { data }
    });

    if let Some(root_idx) = self.root_idx {
      // 1. tree is non-empty, so search for the best sibling
      // to join with the leaf under a new parent node
      let sibling_idx: NodeIdx = find_best_sibling(self, root_idx, &self.nodes[leaf_idx].volume);
      let old_parent_idx = self.nodes[sibling_idx].parent;

      // 2: replace sibling with new_parent, whose children are sibling and leaf
      let new_parent_volume = AABB::join(
        &self.nodes[leaf_idx].volume,
        &self.nodes[sibling_idx].volume
      );

      let new_parent_idx = self.nodes.insert(Node {
        parent : old_parent_idx,
        volume : new_parent_volume,
        kind : NodeKind::Internal {
          child1 : sibling_idx,
          child2 : leaf_idx
        }
      });

      // set new_parent to be the parent of leaf and sibling
      {
        let leaf = &mut self.nodes[leaf_idx];
        leaf.parent = Some(new_parent_idx);
        self.nodes[sibling_idx].parent = Some(new_parent_idx);
      }

      // ensure that old_parent points to new_parent
      if let Some(old_parent_idx) = old_parent_idx {
        // sibling was not the root, so replace
        // it with new_parent under old_parent
        let old_parent = &mut self.nodes[old_parent_idx];
        match old_parent.kind {
          NodeKind::Internal { ref mut child1, ref mut child2 } => {
            if *child1 == sibling_idx { *child1 = new_parent_idx; }
            if *child2 == sibling_idx { *child2 = new_parent_idx; }
          }
          _ => unreachable!("old_parent cannot be a leaf")
        }
      } else {
        // sibling was the root, so
        // make new_parent the root
        self.root_idx = Some(new_parent_idx);
      }

      // 3: walk back up the tree, refitting AABBs
      self.refit_ancestors(leaf_idx);
    } else {
      // tree was empty, use new leaf as root
      self.root_idx = Some(leaf_idx);
    }
  }

  /// refits the volume of a single node to contain its children
  fn refit_node(&mut self, node_idx: NodeIdx) {
    if let NodeKind::Internal { child1, child2 } = self.nodes[node_idx].kind {
      let new_volume = AABB::join(
        &self.nodes[child1].volume,
        &self.nodes[child2].volume
      );

      self.nodes[node_idx].volume = new_volume;
    }
  }

  /// refits the volume of a node and all its ancestors
  fn refit_ancestors(&mut self, leaf_idx: NodeIdx) {
    // walk back up the tree, refitting AABBs
    let mut current_idx = Some(leaf_idx);
    while let Some(idx) = current_idx {
      self.refit_node(idx);
      current_idx = self.nodes[idx].parent;
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

fn tree_cost<D>(tree: &Tree<D>) -> f32 {
  let mut cost = 0.0;
  // we only compare trees with the same leaf nodes,
  // so they are excluded from the cost computation
  for (_, node) in &tree.nodes {
    if let NodeKind::Internal { .. } = node.kind {
      cost += node.volume.surface_area();
    }
  }
  cost
}

use std::cmp::Reverse;
use ordered_float::OrderedFloat;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Candidate {
  /// estimated cost of replacing self with a new
  /// parent node whose children are this and V
  cost_estimate: Reverse<OrderedFloat<f32>>,
  /// increased surface area caused by refitting
  /// all ancestors of this node to include V
  cost_inherit: Reverse<OrderedFloat<f32>>,
  /// index of this candidate
  idx: NodeIdx
}

impl Candidate {
  fn new(cost_estimate: f32, cost_inherit: f32, idx: NodeIdx) -> Self {
    Candidate {
      cost_estimate : Reverse(OrderedFloat(cost_estimate)),
      cost_inherit  : Reverse(OrderedFloat(cost_inherit)),
      idx
    }
  }
}

// increased surface area due to refitting nodes[idx] to include volume
fn delta_cost<D>(tree: &Tree<D>, volume: &AABB, idx: NodeIdx) -> f32 {
  let node = &tree.nodes[idx];
  let old_cost = node.volume.surface_area();
  let new_cost = AABB::join(&node.volume, volume).surface_area();
  return new_cost - old_cost;
}

fn find_best_sibling<D>(tree: &Tree<D>, root_idx: NodeIdx, volume: &AABB) -> NodeIdx {
  // priority queue of candidate nodes
  let mut priority_queue = BinaryHeap::new();
  
  // Cost(Root) = Area(V ∪ Root)
  let root_cost = delta_cost(tree, volume, root_idx);
  let root_candidate = Candidate::new(root_cost, 0.0, root_idx); 
  priority_queue.push(root_candidate);
  
  // branch and bound
  let mut best_idx = root_idx;
  let mut best_cost = root_cost;

  while let Some(current) = priority_queue.pop() {
    // current volume C
    let current_node = &tree.nodes[current.idx];


    // direct cost is the surface area of the new internal
    // node that will be created to hold new leaf and sibling
    //   DirectCost(C) = Area(C ∪ V)
    let direct_cost = AABB::join(volume, &current_node.volume).surface_area();

    // increased surface area caused by refitting C to include V
    //   DeltaCost(C) = Area(C ∪ V) - Area(C)
    let delta_cost = direct_cost - current_node.volume.surface_area();

    // inherited cost is the increased surface area
    // caused by refitting volumes of all ancestors
    let inherited_cost = current.cost_inherit.0.0;

    // TotalCost(C) = DirectCost(C) + InheritedCost(C)
    let total_cost = direct_cost + inherited_cost;

    if total_cost < best_cost {
      best_idx  = current.idx;
      best_cost = total_cost;
    }

    // consider pushing this node's children onto the queue
    if let NodeKind::Internal { child1, child2 } = current_node.kind {
      // For any descendant D of C, we have the following lower bound:
      //   Cost(D)
      //     = DirectCost(D) + InheritedCost(D)                   (by definition)
      //     = Area(D ∪ V)   + InheritedCost(D)                   (by definition)
      //    >= Area(D ∪ V)   + InheritedCost(C) + DeltaCost(C)    (D descendent of C)
      //    >= Area(V)       + InheritedCost(C) + DeltaCost(C)    (monotonicity of area)
      let cost_passed_to_children = inherited_cost + delta_cost;
      let child_lower_bound = volume.surface_area() + cost_passed_to_children;

      if child_lower_bound < best_cost {
        for child in [child1, child2] {
          priority_queue.push(Candidate::new(
            child_lower_bound,
            cost_passed_to_children,
            child
          ));
        }
      }
    }
  }

  best_idx
}