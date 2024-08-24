use nalgebra::{self, Vector2};
use slotmap::{new_key_type, SlotMap};

////////////////////////////////////////////////////////////////////////////////

struct AABB {
  /// lower bound in each axis
  lower_bound: nalgebra::Vector2<f32>,
  /// upper bound in each axis
  upper_bound: nalgebra::Vector2<f32>
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
    // todo
    return false;
  } 
}

////////////////////////////////////////////////////////////////////////////////

enum NodeKind<D> {
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

struct Node<D> {
  parent : Option<NodeIdx>,
  volume : AABB,
  kind   : NodeKind<D>,
}

struct Tree<D> {
  nodes    : SlotMap<NodeIdx, Node<D>>,
  root_idx : Option<NodeIdx>
}

////////////////////////////////////////////////////////////////////////////////

impl<D> Tree<D> {
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
  pub fn insert_leaf(&mut self, object_id: NodeIdx, volume: AABB, data: D) {
    // create new leaf
    let leaf_idx: NodeIdx = self.nodes.insert(Node {
      volume,
      parent : None,
      kind   : NodeKind::Leaf { data }
    });

    // 1: find the best sibling for the new leaf
    let sibling_idx: NodeIdx = panic!("not implemented"); // TODO
    let sibling = &mut self.nodes[sibling_idx];
    let old_parent_idx = sibling.parent;

    // 2: replace sibling with new_parent, whose children are sibling and leaf
    let new_parent_idx = self.nodes.insert(Node {
      parent : old_parent_idx,
      volume : AABB::join(&volume, &sibling.volume),
      kind : NodeKind::Internal {
        child1 : sibling_idx,
        child2 : leaf_idx
      }
    });

    // set new_parent to be the parent of leaf and sibling
    {
      let leaf = &mut self.nodes[leaf_idx];
      leaf.parent = Some(new_parent_idx);
      sibling.parent = Some(new_parent_idx);
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