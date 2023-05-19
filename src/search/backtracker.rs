// /// A backtracker should perform a DFS of states:
// /// 1. keep a history of all states and all actions taken for the state
// /// 2. upon failing, retreat to the last uncompleted state and try a new one
// ///
// /// A BackTracker should be iterate over results
// pub struct DepthFirst<const N: usize, Idx, G> {
//   history: Vec<(State<N, Idx, G>, Vec<usize>)>,
//   pick_domain: PD,
//   pick_tile: PT,
// }

// impl<const N: usize, Idx, G> DepthFirst<N, Idx, G> {
//   pub fn new(init: State<N, Idx, G>) -> Self {}
// }
