mod backtrack;
pub use backtrack::*;
mod naive;
pub use naive::*;
mod restart;
pub use restart::*;
mod state;
pub use state::*;

/// A generic implementation of state for search methods.
///
/// We seperate out getting all applicable actions and picking a specific
/// action here to allow search methods to keep track of the actions taken so
/// far and limit the actions that can be taken.
pub trait State: Sized {
  type Error;
  type Action;

  fn is_goal(&self) -> bool;

  type ActnIter: IntoIterator<Item = Self::Action>;
  fn get_actions(&self) -> Self::ActnIter;

  type PickError: Into<Self::Error>;
  fn pick_action<'a>(
    &'a mut self,
    actions: impl IntoIterator<Item = &'a Self::Action>,
  ) -> Result<Self::Action, Self::PickError>;

  type TakeError: Into<Self::Error>;
  fn take_action(&self, action: &Self::Action) -> Result<Self, Self::TakeError>;
}

pub trait Search<S: State>: Iterator<Item = Result<S, S::Error>> + Sized {
  fn new(start: S) -> Self;
  fn next_valid(&mut self) -> Option<S> {
    self.find_map(|item| item.ok())
  }
}
