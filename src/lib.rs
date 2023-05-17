mod grid;
pub use grid::*;
mod utility;
pub use utility::*;
mod consistency;
pub use consistency::*;
mod tiles;
pub use tiles::*;

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
