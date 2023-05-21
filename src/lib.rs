mod grid;
pub use grid::*;
mod utility;
pub use utility::*;
mod consistency;
pub use consistency::*;
mod search;
pub use search::*;
mod tiles;
pub use tiles::*;
mod sampling;
pub use sampling::*;

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
