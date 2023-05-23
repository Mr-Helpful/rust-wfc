use std::{array, ops::Index};

use crate::{Direction, Tileable};

/// A generic constraint, defined over the indices of tiles and sides.
///
/// We mostly use this to reduce the computation required each time we check a
/// constraint by memo-ising it in a bitset like structure. This also helps us
/// by removing some of the type parameters required to express a constraint,
/// i.e. not having to write out `<T: Tileable<D>, D: Direction>` every time we
/// use the constraint.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Constraint<const N: usize> {
  valid: Vec<[bool; N]>,
  no_tiles: usize,
}

impl<const N: usize> Constraint<N> {
  pub fn new<T: Tileable<D>, D: Direction>(tiles: &[T], sides: &[D; N]) -> Self {
    let no_tiles = tiles.len();
    Self {
      valid: (0..no_tiles)
        .flat_map(|tile_i| {
          (0..no_tiles).map(move |tile_j| {
            array::from_fn(|side| tiles[tile_i].tiles(&tiles[tile_j], &sides[side]))
          })
        })
        .collect(),
      no_tiles,
    }
  }
}

impl<const N: usize> Index<(usize, usize, usize)> for Constraint<N> {
  type Output = bool;
  fn index(&self, (tile_i, tile_j, side): (usize, usize, usize)) -> &Self::Output {
    &self.valid[tile_i * self.no_tiles + tile_j][side]
  }
}
