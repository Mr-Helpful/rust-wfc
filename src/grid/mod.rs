use std::ops::Index;

mod cartesian;
pub use cartesian::*;

/// A discrete grid can be defined in terms of a coordinate system and the
/// neighbours for a given index in the coordinate system.
///
/// For a given coordinate system, there should be a fixed number of neighbours
/// per index, some of which may be unrepresentable (i.e. outside the grid).
///
/// This trait would ideally be better represented with an associated constant,
/// as defining the `Grid`s on a type shouldn't really depend on the number of
/// neighbours specifically, it should either only be implemented once (the
/// associated const approach) or via multiple custom types (to allow for
/// multiple implementations for the same no. neighbours).
pub trait Grid<const N: usize, Idx> {
  fn neighbours(&self, idx: &Idx) -> [Option<Idx>; N];
}

pub trait FromShapeFn<Idx>: Index<Idx> {
  fn from_shape_fn<F: Fn(Idx) -> Self::Output>(shape: Idx, func: F) -> Self;
}

pub trait FromShapeClone<Idx>: FromShapeFn<Idx> + Sized
where
  Self::Output: Clone,
{
  fn from_shape_clone(shape: Idx, item: Self::Output) -> Self {
    Self::from_shape_fn(shape, |_| item.clone())
  }
}

pub trait FromShapeDefault<Idx>: FromShapeFn<Idx> + Sized
where
  Self::Output: Default,
{
  fn from_shape_default(shape: Idx) -> Self {
    Self::from_shape_fn(shape, |_| Default::default())
  }
}
