mod utils;
mod wfc;
pub use wfc::*;
mod grid;
pub use grid::*;
mod methods;
pub use methods::*;

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
