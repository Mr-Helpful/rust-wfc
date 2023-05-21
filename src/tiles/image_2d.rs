use ndarray::{prelude::Array2, s, ArrayView1};

use super::{Direction, Tileable};

pub enum Side2D {
  Up,
  Right,
  Down,
  Left,
}

impl Direction for Side2D {
  fn opposite(&self) -> Self {
    match self {
      Self::Up => Self::Down,
      Self::Down => Self::Up,
      Self::Left => Self::Right,
      Self::Right => Self::Left,
    }
  }
}

pub struct ImageEdge2D<T>(Array2<T>);

impl<T> ImageEdge2D<T> {
  /// Fetches the 1D edge of an image corresponding to the side
  fn edge<'a>(&'a self, side: &Side2D) -> ArrayView1<'a, T> {
    use Side2D::*;
    match side {
      Up => self.0.slice(s![0, ..]),
      Right => self.0.slice(s![.., -1]),
      Down => self.0.slice(s![-1, ..;-1]),
      Left => self.0.slice(s![..;-1, 0]),
    }
  }
}

impl<T: Eq> Tileable<Side2D> for ImageEdge2D<T> {
  fn tiles(&self, other: &Self, side: &Side2D) -> bool {
    self.edge(side).slice(s![..;-1]) == other.edge(&side.opposite())
  }
}

pub struct ImageGrid2D<T>(Array2<T>);

impl<T: Clone> ImageGrid2D<T> {
  /// Computes the overlap for a direction, i.e. the overlap between the tile
  /// and another tile moved one unit in that direction
  fn overlap(&self, side: &Side2D) -> Array2<T> {
    use Side2D::*;
    match side {
      Up => self.0.slice(s![.., ..-1]).to_owned(),
      Right => self.0.slice(s![1..;-1, ..]).t().to_owned(),
      Down => self.0.slice(s![..;-1, 1..;-1]).to_owned(),
      Left => self.0.slice(s![..-1, ..;-1]).t().to_owned(),
    }
  }
}

impl<T: Eq + Clone> Tileable<Side2D> for ImageGrid2D<T> {
  fn tiles(&self, other: &Self, side: &Side2D) -> bool {
    self.overlap(side).slice(s![..;-1, ..]) == other.overlap(&side.opposite())
  }
}
