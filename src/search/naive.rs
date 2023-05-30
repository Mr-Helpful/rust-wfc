use crate::{Search, State};

/// Generates a single failure/success and then None repeatedly
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Naive<S: State> {
  item: Option<S>,
}

impl<S: State> Iterator for Naive<S> {
  type Item = Result<S, S::Error>;
  fn next(&mut self) -> Option<Self::Item> {
    let mut state = self.item.take()?;

    while !state.is_goal() {
      let actns: Vec<_> = state.get_actions().into_iter().collect();
      let choice = match state.pick_action(actns.iter()) {
        Err(e) => return Some(Err(e.into())),
        Ok(choice) => choice,
      };

      state = match state.take_action(&choice) {
        Err(e) => return Some(Err(e.into())),
        Ok(state) => state,
      };
    }

    Some(Ok(state))
  }
}

impl<S: State> Search<S> for Naive<S> {
  fn new(start: S) -> Self {
    Self { item: Some(start) }
  }
}
