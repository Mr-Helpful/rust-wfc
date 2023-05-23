use super::{Direction, End, Side, Tileable};
use ndarray::{Array, ArrayView, Axis, Dimension, Slice};

pub struct ImageEdge<T, D>(pub Array<T, D>);

impl<T, D> From<Array<T, D>> for ImageEdge<T, D> {
  fn from(value: Array<T, D>) -> Self {
    Self(value)
  }
}

impl<T, D: Dimension> ImageEdge<T, D> {
  pub(crate) fn edge_of(&self, side: &Side) -> Option<ArrayView<T, D>> {
    for i in 0..self.0.ndim() {
      match side {
        Side(Axis(j), End::Low) if &i == j => {
          return Some(self.0.slice_axis(Axis(*j), Slice::from(0..1)))
        }
        Side(Axis(j), End::High) if &i == j => {
          return Some(self.0.slice_axis(Axis(*j), Slice::from(-1..)))
        }
        _ => {}
      }
    }

    None
  }
}

impl<T: PartialEq, D: Dimension> Tileable<Side> for ImageEdge<T, D> {
  fn tiles(&self, other: &Self, side: &Side) -> bool {
    self
      .edge_of(side)
      .zip(other.edge_of(&side.opposite()))
      .map_or(false, |(side0, side1)| side0 == side1)
  }
}
