use super::Sampler;
use rand::{distributions::WeightedIndex, Rng};

/// Uses each entry to fetch the probability at which it should be returned
///
/// For example, with the weights and entries:
/// ```
/// let weights = [0.1, 0.2, 0.3, 0.4];
/// let entries = [3, 1, 0];
/// ```
/// weighted sampling will return:
/// - `3` with probability `0.4`
/// - `1` with probability `0.2`
/// - `0` with probability `0.1`
#[derive(Clone, Debug, Default)]
pub struct Weighted<'a, R> {
  rng: R,
  weights: &'a [f64],
}

impl<'a, R0, R1> PartialEq<Weighted<'a, R1>> for Weighted<'a, R0> {
  fn eq(&self, other: &Weighted<'a, R1>) -> bool {
    self.weights == other.weights
  }
}
impl<'a, R> Eq for Weighted<'a, R> {}

impl<'a, R: Rng> Sampler for Weighted<'a, R> {
  fn sample(&mut self, entries: &[usize]) -> usize {
    assert!(
      !entries.is_empty(),
      "Should sample from a non-empty collection"
    );
    assert!(
      &self.weights.len() > entries.iter().max().unwrap_or(&0),
      "The #of weights should be greater than the maximum item"
    );

    let weights: Vec<_> = entries.iter().map(|&i| self.weights[i]).collect();
    let dist = WeightedIndex::new(weights).unwrap();
    entries[self.rng.sample(dist)]
  }
}
