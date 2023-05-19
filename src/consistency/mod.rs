mod space;
use space::Space;
mod domain;
use domain::Domain;
mod errors;
pub use errors::*;
mod ac3;
pub use ac3::*;

pub type State<const N: usize, Idx> = Space<Idx, Domain<N>>;
