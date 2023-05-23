use super::Direction;
use ndarray::Axis;

pub enum End {
  Low,
  High,
}

pub struct Side(pub Axis, pub End);

impl Direction for Side {
  fn opposite(&self) -> Self {
    match self {
      Side(ax, End::Low) => Self(*ax, End::High),
      Side(ax, End::High) => Self(*ax, End::Low),
    }
  }
}
