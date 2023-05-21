use crate::Sampler;
use rand::distributions::WeightedIndex;
use rand::Rng;

#[derive(Clone)]
pub struct Weighted<'a, R: Rng> {
  rng: R,
  weights: &'a [f64],
}

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
