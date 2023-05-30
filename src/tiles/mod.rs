mod images;
pub use images::{ImageEdge, ImageEnd, ImageGrid, ImageSide};
mod words;
pub use words::{Word, WordSide};

pub trait Direction {
  /// Fetches the direction immediately opposite this one
  fn opposite(&self) -> Self;
}

pub trait Tileable<D: Direction> {
  /// Returns whether a tile can be placed next to this one in a given direction
  fn tiles(&self, other: &Self, side: &D) -> bool;
}
