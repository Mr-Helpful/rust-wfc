pub trait Direction {
  /** Fetches the direction immediately opposite this one */
  fn opposite(&self) -> Self;
}
