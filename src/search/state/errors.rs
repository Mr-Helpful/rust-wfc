use crate::AC3Error;

pub enum WFCError<Idx> {
  GetActionError,
  PickActionError(String),
  TakeActionError(AC3Error<Idx>),
}

impl<Idx> From<AC3Error<Idx>> for WFCError<Idx> {
  fn from(value: AC3Error<Idx>) -> Self {
    Self::TakeActionError(value)
  }
}
