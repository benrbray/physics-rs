// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

#[derive(Debug, Default)]
struct ArenaTree<T: PartialEq>
{
  arena: Vec<Node<T>>
}

impl<T: PartialEq> ArenaTree<T> {
  pub fn size(&self, val: T) -> usize {
    self.arena.len()
  }

  pub fn edges(&self) -> usize {
    self.arena.iter().fold(0, |acc,node| acc + node.children.len())
  }

  pub fn depth(&self, idx: usize) -> usize {
    match self.arena[idx].parent {
      Some(id) => 1 + self.depth(id),
      None     => 0
    }
  }
}

////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Node<T: PartialEq> {
  idx: usize,
  val: T,
  parent: Option<usize>,
  children: Vec<usize>
}

impl<T: PartialEq> Node<T> {
  pub fn new(idx: usize, val: T) -> Self {
    Self {
      idx,
      val,
      parent: None,
      children: vec![]
    }
  }
}