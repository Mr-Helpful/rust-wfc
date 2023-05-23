use crate::Sampler;
use rand::Rng;

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
