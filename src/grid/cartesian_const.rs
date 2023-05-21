/// A generic N dimensional cartesian grid, for both finite and wrapped grids
use super::Grid;

pub struct Cartesian<const N: usize>([usize; N]);

impl<const N: usize> Grid<{ 2 * N }, [usize; N]> for Cartesian<N> {
  fn neighbours(&self, idx: &[usize; N]) -> [Option<[usize; N]>; 2 * N] {
    let mut result = [None; 2 * N];
    if self.0.into_iter().any(|w| w == 0) {
      return result;
    }

    for i in 0..N {
      if idx[i] > 1 {
        let mut n_idx = idx.clone();
        n_idx[i] -= 1;
        result[i].insert(n_idx)
      }
    }

    for i in 0..N {
      if idx[i] < self.0[i] - 1 {
        let mut n_idx = idx.clone();
        n_idx[i] += 1;
        result[i + N].insert(n_idx)
      }
    }

    result
  }
}

pub struct CartesianWrap<const N: usize>([usize; N]);

impl<const N: usize> Grid<{ 2 * N }, [usize; N]> for Cartesian<N> {
  fn neighbours(&self, idx: &[usize; N]) -> [Option<[usize; N]>; 2 * N] {
    let mut result = [None; 2 * N];
    if self.0.into_iter().any(|w| w == 0) {
      return result;
    }

    for i in 0..N {
      let mut n_idx = idx.clone();
      n_idx[i] = (n_idx[i] + (self.0[i] - 1)) % self.0[i];
      result[i].insert(n_idx);
    }

    for i in 0..N {
      let mut n_idx = idx.clone();
      n_idx[i] = (n_idx[i] + 1) % self.0[i];
      result[i + N].insert(n_idx);
    }

    result
  }
}
