use std::ops::{Index, IndexMut};

use crate::Grid;

pub struct Grid2D<T>(Vec<Vec<T>>);

impl<T> Index<(usize, usize)> for Grid2D<T> {
  type Output = T;
  fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
    &self.0[j][i]
  }
}

impl<T> IndexMut<(usize, usize)> for Grid2D<T> {
  fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
    &mut self.0[i][j]
  }
}

impl<T> Grid<4, (usize, usize)> for Grid2D<T> {
  fn neighbours(&self, (i, j): &(usize, usize)) -> [Option<(usize, usize)>; 4] {
    let w = self.0.get(0).map(|row| row.len()).unwrap_or(0);
    let h = self.0.len();
    [
      (*j > 0).then_some((*i, j - 1)),
      (*i > 0).then_some((i - 1, *j)),
      (*i < w - 1).then_some((i + 1, *j)),
      (*j < h - 1).then_some((*i, j + 1)),
    ]
  }
}
