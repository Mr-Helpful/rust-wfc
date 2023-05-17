use super::Grid;

pub struct Cartesian2D((usize, usize));

impl Grid<4, (usize, usize)> for Cartesian2D {
  fn neighbours(&self, (i, j): &(usize, usize)) -> [Option<(usize, usize)>; 4] {
    let (w, h) = self.0;
    [
      (*j > 0).then_some((*i, j - 1)),
      (*i > 0).then_some((i - 1, *j)),
      (*i < w - 1).then_some((i + 1, *j)),
      (*j < h - 1).then_some((*i, j + 1)),
    ]
  }
}

pub struct CartesianWrap2D((usize, usize));

impl Grid<4, (usize, usize)> for CartesianWrap2D {
  fn neighbours(&self, (i, j): &(usize, usize)) -> [Option<(usize, usize)>; 4] {
    let (w, h) = self.0;
    [
      Some((*i, (j + (h - 1)) % h)),
      Some(((i + (w - 1)) % w, *j)),
      Some((*i, (j + 1) % h)),
      Some(((i + 1) % w, *j)),
    ]
  }
}
