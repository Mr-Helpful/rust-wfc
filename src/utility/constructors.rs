use std::ops::Index;

pub trait FromFnCount<T>: FromIterator<T> {
  fn from_fn_count(count: usize, f: fn(usize) -> T) -> Self {
    (0..count).map(f).collect()
  }
}

pub trait FromShapeFn<Idx>: Index<Idx> {
  fn from_shape_fn(
    shape: Idx,
    func: impl Fn(Idx) -> Self::Output,
  ) -> Self;
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
