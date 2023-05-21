use crate::{Search, State};

/// A backtracker should perform a DFS of states:
/// 1. keep a history of all states and all actions taken for the state
/// 2. upon failing, retreat to the last uncompleted state and try a new one
pub struct DepthFirst<S: State> {
  history: Vec<(S, Vec<S::Action>)>,
}

impl<S: State> Iterator for DepthFirst<S>
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

impl<S: State> Search<S> for DepthFirst<S>
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
