use super::Direction;
use ndarray::Axis;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum End {
  Low,
  High,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Side(pub Axis, pub End);

impl Direction for Side {
  fn opposite(&self) -> Self {
    match self {
      Side(ax, End::Low) => Self(*ax, End::High),
      Side(ax, End::High) => Self(*ax, End::Low),
    }
  }
}
