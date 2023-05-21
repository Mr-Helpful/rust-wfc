use crate::Sampler;

#[derive(Clone, Copy)]
pub struct First;

impl Sampler for First {
  fn sample(&mut self, entries: &[usize]) -> usize {
    assert!(
      !entries.is_empty(),
      "Should sample from a non-empty collection"
    );
    entries[0]
  }
}
