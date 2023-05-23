/// Cartesian and Wrapped grids for up to 5 dimensions
/// I also export the macros for creating these grids in case anyone wants
/// higher dimensions
use super::Grid;

#[macro_export]
macro_rules! cartesian_grid {
  ($name:ident, $ndims:literal) => {
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

cartesian_grid!(Cartesian1D, 1);
cartesian_grid!(Cartesian2D, 2);
cartesian_grid!(Cartesian3D, 3);
cartesian_grid!(Cartesian4D, 4);
cartesian_grid!(Cartesian5D, 5);

#[macro_export]
macro_rules! cartesian_wrapped_grid {
  ($name:ident, $ndims:literal) => {
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

cartesian_wrapped_grid!(CartesianWrapped1D, 1);
cartesian_wrapped_grid!(CartesianWrapped2D, 2);
cartesian_wrapped_grid!(CartesianWrapped3D, 3);
cartesian_wrapped_grid!(CartesianWrapped4D, 4);
cartesian_wrapped_grid!(CartesianWrapped5D, 5);
