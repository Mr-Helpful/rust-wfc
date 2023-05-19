mod cartesian_2d;
pub use cartesian_2d::*;

/// This is just a nice way of implementing ND cartesian grids, but it can't be
/// implemented until const parameters of the form `{2 * N}` are introduced,
/// i.e. the `const_evaluatable_checked` feature.
// mod cartesian;
// pub use cartesian::*;

/// A discrete grid can be defined in terms of a coordinate system and the
/// neighbours for a given index in the coordinate system.
///
/// For a given coordinate system, there should be a fixed number of neighbours
/// per index, some of which may be unrepresentable (i.e. outside the grid).
///
/// This trait would ideally be better represented with an associated constant,
/// as defining the `Grid`s on a type shouldn't really depend on the number of
/// neighbours specifically, it should either only be implemented once (the
/// associated const approach) or via multiple custom types (to allow for
/// multiple implementations for the same no. neighbours).
pub trait Grid<const N: usize, Idx> {
  fn neighbours(&self, idx: &Idx) -> [Option<Idx>; N];

  /// Finds the updates to propagate for each direction in the given grid
  /// for a given collection of tiles changed
  fn updates_for(
    &self,
    idx: &Idx,
    tiles: impl IntoIterator<Item = usize>,
  ) -> Vec<(Idx, usize, usize)> {
    tiles
      .into_iter()
      .flat_map(|tile| {
        self
          .neighbours(idx)
          .into_iter()
          .enumerate()
          .filter_map(move |(side, optn)| optn.map(|idx| (idx, side, tile)))
      })
      .collect()
  }
}
