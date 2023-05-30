mod backtrack;
pub use backtrack::*;
mod naive;
pub use naive::*;
mod restart;
pub use restart::*;
mod state;
pub use state::*;
mod errors;
pub use errors::WFCError;

/// A generic implementation of state for search methods.
///
/// We seperate out getting all applicable actions and picking a specific
/// action here to allow search methods to keep track of the actions taken so
/// far and limit the actions that can be taken.
pub trait State: Sized {
  /// A supertype for all errors thrown during state generation<br>
  /// All errors should be convertable into this type
  type Error;
  /// A type for actions that can be taken on this state<br>
  /// We should be able to transition from this state with anything satisfying
  /// the action type
  type Action;

  /// Whether the current state is an acceptable final state
  fn is_goal(&self) -> bool;

  /// An iterable over all possible actions
  type ActnIter: IntoIterator<Item = Self::Action>;
  /// Fetches all possible actions for this state
  fn get_actions(&self) -> Self::ActnIter;

  /// The error produced whilst choosing a tile to narrow a domain to
  type PickError: Into<Self::Error>;
  /// Chooses an action from all available actions<br>
  /// This can either be deterministic or non-deterministic (hence the `&mut`)
  fn pick_action<'a>(
    &'a mut self,
    actions: impl IntoIterator<Item = &'a Self::Action>,
  ) -> Result<Self::Action, Self::PickError>;

  /// The error produced whilst attempting to transition to a new state
  type TakeError: Into<Self::Error>;
  /// Takes an action and transitions into a new state<br>
  /// This should produce a new state from a reference to this state (i.e. via
  /// `clone`)
  fn take_action(&self, action: &Self::Action) -> Result<Self, Self::TakeError>;
}

pub trait Search<S: State>: Iterator<Item = Result<S, S::Error>> + Sized {
  fn new(start: S) -> Self;
  fn next_valid(&mut self) -> Option<S> {
    self.find_map(|item| item.ok())
  }
}
