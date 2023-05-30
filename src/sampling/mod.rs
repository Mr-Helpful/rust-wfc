/// A bit of a preface to this, I'd much prefer to just create a `Distribution`
/// like those from the `rand` crate, but I can't for two main reasons:
/// - I'd like to make `Clone` easier to use for samplers
/// - Sampling needs to be over a collection rather than on a RV
mod first;
pub use first::First;
mod uniform;
pub use uniform::Uniform;
mod weighted;
pub use weighted::Weighted;

pub trait Sampler {
  fn sample(&mut self, entries: &[usize]) -> usize;
}
