/// Cartesian and Wrapped grids for up to 5 dimensions
/// I also export the macros for creating these grids in case anyone wants
/// higher dimensions
use super::Grid;

#[macro_export]
macro_rules! cartesian_grid {
  ($name:ident, $ndims:literal) => {
    /// A cartesian grid
    ///
    /// The grid has a defined size, outside of which indices aren't defined
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct $name([usize; $ndims]);

    impl Grid<{ 2 * $ndims }, [usize; $ndims]> for $name {
      fn neighbours(&self, idx: &[usize; $ndims]) -> [Option<[usize; $ndims]>; 2 * $ndims] {
        let mut result = [None; 2 * $ndims];
        if self.0.into_iter().any(|w| w == 0) {
          return result;
        }

        for i in 0..$ndims {
          if idx[i] > 1 {
            let mut n_idx = idx.clone();
            n_idx[i] -= 1;
            result[i] = Some(n_idx);
          }
        }

        for i in 0..$ndims {
          if idx[i] < self.0[i] - 1 {
            let mut n_idx = idx.clone();
            n_idx[i] += 1;
            result[i + $ndims] = Some(n_idx);
          }
        }

        result
      }
    }
  };
}

cartesian_grid!(Cartesian1, 1);
cartesian_grid!(Cartesian2, 2);
cartesian_grid!(Cartesian3, 3);
cartesian_grid!(Cartesian4, 4);
cartesian_grid!(Cartesian5, 5);

#[macro_export]
macro_rules! cartesian_wrapped_grid {
  ($name:ident, $ndims:literal) => {
    /// A cartesian wrapped grid
    ///
    /// The grid has a defined size, outside of which indices are wrapped around
    /// back around to the opposite edge
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct $name([usize; $ndims]);

    impl Grid<{ 2 * $ndims }, [usize; $ndims]> for $name {
      fn neighbours(&self, idx: &[usize; $ndims]) -> [Option<[usize; $ndims]>; 2 * $ndims] {
        let mut result = [None; 2 * $ndims];
        if self.0.into_iter().any(|w| w == 0) {
          return result;
        }

        for i in 0..$ndims {
          let mut n_idx = idx.clone();
          n_idx[i] = (n_idx[i] + (self.0[i] - 1)) % self.0[i];
          result[i] = Some(n_idx);
        }

        for i in 0..$ndims {
          let mut n_idx = idx.clone();
          n_idx[i] = (n_idx[i] + 1) % self.0[i];
          result[i + $ndims] = Some(n_idx);
        }

        result
      }
    }
  };
}

cartesian_wrapped_grid!(Wrapped1, 1);
cartesian_wrapped_grid!(Wrapped2, 2);
cartesian_wrapped_grid!(Wrapped3, 3);
cartesian_wrapped_grid!(Wrapped4, 4);
cartesian_wrapped_grid!(Wrapped5, 5);
