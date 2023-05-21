use crate::{Search, State};

pub struct Restart<S: State> {
  item: S,
}

impl<S: State + Clone> Iterator for Restart<S> {
  type Item = Result<S, S::Error>;
  fn next(&mut self) -> Option<Self::Item> {
    let mut state = self.item.clone();

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

impl<S: State + Clone> Search<S> for Restart<S> {
  fn new(start: S) -> Self {
    Self { item: start }
  }
}
