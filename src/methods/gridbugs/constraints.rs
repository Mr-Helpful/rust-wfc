use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Adjacency<D>(Vec<HashMap<D, HashSet<usize>>>);

impl<D: Clone + Hash + Eq> Adjacency<D> {
  pub fn new<T>(tileset: &[T], directions: &[D], valid: fn(&T, &D, &T) -> bool) -> Self {
    Adjacency(
      tileset
        .iter()
        .enumerate()
        .map(|(_, tile0)| {
          directions
            .iter()
            .map(|side| {
              (
                side.clone(),
                tileset
                  .iter()
                  .enumerate()
                  .filter(|(_, tile1)| valid(tile0, side, tile1))
                  .map(|(k, _)| k)
                  .collect(),
              )
            })
            .collect()
        })
        .collect(),
    )
  }
}

impl<D: Hash + Eq> Adjacency<D> {
  pub fn consistent(&self, x: usize, side: &D) -> &HashSet<usize> {
    self
      .0
      .get(x)
      .and_then(|sides| sides.get(side))
      .expect("Consitency should only be checked against present tiles")
  }

  pub fn is_valid(&self, (x, y): (usize, usize), side: &D) -> bool {
    self.consistent(x, side).contains(&y)
  }
}
