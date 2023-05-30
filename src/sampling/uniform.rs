use crate::Sampler;
use rand::Rng;

/// Randomly takes any entry in the collection with equal likelihood
#[derive(Clone, Debug, Default)]
pub struct Uniform<R: Rng> {
  rng: R,
}

impl<R: Rng> Sampler for Uniform<R> {
  fn sample(&mut self, entries: &[usize]) -> usize {
    assert!(
      !entries.is_empty(),
      "Should sample from a non-empty collection"
    );
    entries[self.rng.gen_range(0..entries.len())]
  }
}
