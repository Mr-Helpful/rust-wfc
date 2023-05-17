use super::Direction;
use ndarray::prelude::*;
type Img<T> = Array2<T>;

pub enum Side {
  Up,
  Right,
  Down,
  Left,
}

impl Direction for Side {
  fn opposite(&self) -> Self {
    match self {
      Self::Up => Self::Down,
      Self::Down => Self::Up,
      Self::Left => Self::Right,
      Self::Right => Self::Left,
    }
  }
}

impl Side {
  /** Fetches the 1D edge of an image corresponding to the side */
  fn edge<'a, T>(&'a self, img: &'a Img<T>) -> ArrayView1<'a, T> {
    match self {
      Self::Up => img.slice(s![0, ..]),
      Self::Right => img.slice(s![.., -1]),
      Self::Down => img.slice(s![-1, ..;-1]),
      Self::Left => img.slice(s![..;-1, 0]),
    }
  }

  /** Tests whether, when placed side by side, the tile sides are equal */
  pub fn edge_matches<T: Eq>(&self, img0: &Img<T>, img1: &Img<T>) -> bool {
    self.edge(img0).slice(s![..;-1]) == self.opposite().edge(img1)
  }

  /** Computes the overlap for a direction, i.e. the overlap between the tile
  and another tile moved one unit in that direction
   */
  fn overlap<T: Clone>(&self, img: &Img<T>) -> Array2<T> {
    match self {
      Self::Up => img.slice(s![.., ..-1]).to_owned(),
      Self::Right => img.slice(s![1..;-1, ..]).t().to_owned(),
      Self::Down => img.slice(s![..;-1, 1..;-1]).to_owned(),
      Self::Left => img.slice(s![..-1, ..;-1]).t().to_owned(),
    }
  }

  /** Tests whether the intersection between two tiles, placed next to each
  other, matches.
  */
  pub fn overlap_matches<T: Clone + Eq>(&self, img0: &Img<T>, img1: &Img<T>) -> bool {
    self.overlap(img0).slice(s![..;-1, ..]) == self.opposite().overlap(img1)
  }
}
