use crate::Sampler;
use rand::distributions::WeightedIndex;
use rand::Rng;

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
    let dist = WeightedIndex::new(&weights).unwrap();
    self.rng.sample(dist)
  }
}
