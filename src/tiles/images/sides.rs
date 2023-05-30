use super::Direction;
use ndarray::Axis;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageEnd {
  Low,
  High,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImageSide(pub Axis, pub ImageEnd);

impl Direction for ImageSide {
  fn opposite(&self) -> Self {
    match self {
      ImageSide(ax, ImageEnd::Low) => Self(*ax, ImageEnd::High),
      ImageSide(ax, ImageEnd::High) => Self(*ax, ImageEnd::Low),
    }
  }
}
