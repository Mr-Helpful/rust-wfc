mod image_2d;
pub use image_2d::*;
mod words;
pub use words::*;

pub trait Direction {
  /** Fetches the direction immediately opposite this one */
  fn opposite(&self) -> Self;
}

pub trait Tileable<D: Direction> {
  /// Returns whether a tile can be placed next to this one in a given direction
  fn fits_together(&self, other: &Self, side: &D) -> bool;
}
