# Wave Function Collapse

This project aims to implement wave function collapse in about the most generic way possible.

## TODO

- [ ] implement tile selection
  - [ ] tile selection heuristics
  - [ ] randomised tile choices
  - [ ] weighted tile choices
- [ ] implement backtracking/failure heuristics
  - [ ] create trait for handling contradictions
  - [ ] implement propagating approach
  - [ ] implement resetting approach
  - [ ] implement backtracking approach

## The algorithm

Wave function collapse uses applies a constraint based approach to procedural generation and roughly follows the algorithm of:

```txt
fn wfc(tiles, shape, valid_next_to) {
  domains = array of every tile in `tiles` with `shape` dimensions

  while not all domains collapsed {
    choose some domain to collapse
    collapse the domain to a single value it contains

    use the AC3 algorithm to make the domain consistent
    if the AC3 algorithm leads to an empty domain (a contradiction) {
      apply some algorithm to handle the error
    }
  }

  return domains
}
```

However, we generalise it in two key ways:

## Custom Tiles

We allow for customised definitions of tiles by defining the [`Tileable`](src/tiles/mod.rs#L11) trait, that tests whether two tiles can be placed side by side.

This allows for different definitions of a tile, such as:

- Images compared on edges: [`ImageEdge2D`](src/tiles/image_2d.rs#L23)
- Images compared on a fixed grid size: [`ImageGrid2D`](src/tiles/image_2d.rs#L44)
- Words that form a portmanteau, i.e. "_sailor_" + "_lorded_" share "_lor_": [`Word`](src/tiles/custom/words.rs#L17)

## Custom Grids

We also allow for customised definitions of grids by defining the [`Grid`](src/grid/mod.rs#L15) trait, that generates all the neighbours an index can have.

This allows for different definitions of a grid, such as:

- N-Dimensional cartesian grids: [`Cartesian2D`](src/grid/cartesian_2d.rs#L3)
- N-Dimensional cartesian modulo grids [`CartesianWrap2D`](src/grid/cartesian_2d.rs#L17)
