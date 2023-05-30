use crate::{Search, State};

/// Performs a Depth First Search of possible states.
///
/// This will generate states until either an error is thrown or a goal state
/// is generated. This final state/error will then be output and the search
/// will backtrack to the last non-goal/valid state and make a different choice
/// for its action.
///
/// For example, we could get the trace:
/// 1. @state_0, actions {a, b}
/// 1. state_0 --{a}-> state_1
/// 1. @state_1, actions {c}
/// 1. state_1 --{c}-> error
/// 1. output Err(error), backtrack to state_1
/// 1. @state_1, actions {}
/// 1. no actions left, backtrack to state_0
/// 1. @state_0, actions {b}
/// 1. state_0 --{b}-> state_2
/// 1. @state_2, goal reached
/// 1. output Ok(state_2), backtrack to state_0
/// 1. no actions left, backtrack and finish
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Backtrack<S: State> {
  history: Vec<(S, Vec<S::Action>)>,
}

impl<S: State> Iterator for Backtrack<S>
where
  S::Action: Eq,
{
  type Item = Result<S, S::Error>;
  fn next(&mut self) -> Option<Self::Item> {
    let (mut state, mut actns) = self.history.pop()?;
    if actns.is_empty() {
      return self.next();
    }

    // get the action to take
    let choice = match state.pick_action(actns.iter()) {
      Err(e) => return Some(Err(e.into())),
      Ok(choice) => choice,
    };
    let i = actns.iter().position(|actn| actn == &choice).unwrap();
    let actn = actns.swap_remove(i);

    // get the new state for this action
    let result = state.take_action(&actn);
    self.history.push((state, actns));
    let new_state = match result {
      Err(e) => return Some(Err(e.into())),
      Ok(new_state) => new_state,
    };

    // get the new actions for this state
    let new_actns = new_state.get_actions().into_iter().collect();
    self.history.push((new_state, new_actns));
    self.next()
  }
}

impl<S: State> Search<S> for Backtrack<S>
where
  S::Action: Eq,
{
  fn new(start: S) -> Self {
    let actns = start.get_actions().into_iter().collect();
    Self {
      history: vec![(start, actns)],
    }
  }
}
