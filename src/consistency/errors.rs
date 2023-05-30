use std::{collections::TryReserveError, fmt::Display};

use super::super::utility::WorkerBagError;

/// An error created during the process of AC3 constraint propagation
/// Keeps track of both the domain and item we were attempting to assign
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AC3Error<Idx> {
  domain: Idx,
  item: usize,
  kind: AC3ErrorKind,
}

impl<Idx: Display> Display for AC3Error<Idx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.kind {
      AC3ErrorKind::InvalidChoice => write!(
        f,
        "Item #{} is not valid in the domain at {}",
        self.item, self.domain
      ),
      AC3ErrorKind::InconsistentChoice => write!(
        f,
        "Assigning item #{} to the domain at {} leads to a contradiction",
        self.item, self.domain
      ),
      AC3ErrorKind::BufferFilled(_) => write!(
        f,
        "Filled worker queue whilst setting domain at {} to item #{}",
        self.domain, self.item
      ),
    }
  }
}

impl<Idx> AC3Error<Idx> {
  pub fn new(domain: Idx, item: usize, kind: impl Into<AC3ErrorKind>) -> Self {
    Self {
      domain,
      item,
      kind: kind.into(),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum AC3ErrorKind {
  #[default]
  InvalidChoice,
  InconsistentChoice,
  BufferFilled(TryReserveError),
}

impl From<WorkerBagError<AC3ErrorKind>> for AC3ErrorKind {
  fn from(value: WorkerBagError<AC3ErrorKind>) -> Self {
    match value {
      WorkerBagError::WorkerError(e) => e,
      WorkerBagError::BagFullError(e) => AC3ErrorKind::BufferFilled(e),
    }
  }
}
