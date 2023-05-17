use rayon::prelude::*;

pub enum WFCError {
  EmptyDomain(usize, usize),
}

type Grid<T> = Vec<Vec<T>>;

pub fn arc3_resolve<Tile: Clone + Send + Sync>(
  mut grid: Grid<Vec<Tile>>,
  to_check: impl Iterator<Item = (usize, usize)> + Clone + Sync,
  constraint: fn(&Tile, &Tile) -> bool,
) -> Result<Vec<Vec<Vec<Tile>>>, WFCError> {
  use WFCError::EmptyDomain;
  let (w, h) = (grid[0].len(), grid.len());
  let mut unresolved = false;

  while !unresolved {
    let mut new_grid: Vec<Vec<_>> = (0..w).map(|_| (0..h).map(|_| vec![]).collect()).collect();

    unresolved = new_grid
      .par_iter_mut()
      .enumerate()
      .map(|(j, new_row)| {
        new_row
          .par_iter_mut()
          .enumerate()
          .map(|(i, new_cell)| {
            let mut unresolved = false;
            for tile in &grid[j][i] {
              let consistent = to_check.clone().any(|(di, dj)| {
                let (i, j) = (i + di, j + dj);
                !((0..w).contains(&i) & (0..h).contains(&j))
                  && grid[j][i].iter().any(|other| constraint(tile, other))
              });

              if consistent {
                new_cell.push(tile.clone())
              } else {
                unresolved = true
              }
            }

            if new_cell.is_empty() {
              Err(EmptyDomain(i, j))
            } else {
              Ok(unresolved)
            }
          })
          .try_reduce(|| false, |x, y| Ok(x | y))
      })
      .try_reduce(|| false, |x, y| Ok(x | y))?;

    grid = new_grid
  }

  Ok(grid)
}

// pub fn wfc<Tile: Clone>(tiles: Vec<Tile>, (w, h): (usize, usize)) -> Result<Grid<Tile>, WFCError> {
//   let grid = clone_to_grid(tiles, (w, h));

//   loop {}
// }
