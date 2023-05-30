use super::{Direction, ImageEnd, ImageSide, Tileable};
use ndarray::{Array, ArrayView, Axis, Dimension, Slice};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageEdge<T, D: Dimension>(pub Array<T, D>);

impl<T, D: Dimension> From<Array<T, D>> for ImageEdge<T, D> {
  fn from(value: Array<T, D>) -> Self {
    Self(value)
  }
}

impl<T, D: Dimension> ImageEdge<T, D> {
  fn edge_of(&self, side: &ImageSide) -> Option<ArrayView<T, D>> {
    for i in 0..self.0.ndim() {
      match side {
        ImageSide(Axis(j), ImageEnd::Low) if &i == j => {
          return Some(self.0.slice_axis(Axis(*j), Slice::from(0..1)))
        }
        ImageSide(Axis(j), ImageEnd::High) if &i == j => {
          return Some(self.0.slice_axis(Axis(*j), Slice::from(-1..)))
        }
        _ => {}
      }
    }

    None
  }
}

impl<T: PartialEq, D: Dimension> Tileable<ImageSide> for ImageEdge<T, D> {
  fn tiles(&self, other: &Self, side: &ImageSide) -> bool {
    self
      .edge_of(side)
      .zip(other.edge_of(&side.opposite()))
      .map_or(false, |(side0, side1)| side0 == side1)
  }
}
