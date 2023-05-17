# The Wave Function Collapse Algorithm

This project consists of broad implementation of wave function collapse and other similar algorithms, all of which follow the rough pattern of:

```rs
fn resolve<Tile: Clone>(
  grid: Vec<Vec<Vec<Tile>>>
  constraint: fn(Tile, Tile) -> bool
) -> Result<Vec<Vec<Vec<Tile>>>, WFCError> {
  let mut unresolved = false;

  for row in grid.iter_mut() {
    for cell in grid.iter_mut() {
      cell = cell.filter_w
    }
  }
}

fn wfc<Tile: Clone>(
  tiles: Vec<Tile>, (w, h): (usize, usize)
) -> Result<Vec<Vec<Tile>>, WFCError> {
  let grid = clone_to_grid(tiles, (w, h));
  
  loop {

  }
}
```
