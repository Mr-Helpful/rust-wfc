use super::Sampler;

/// Takes the first entry from the given collection
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
