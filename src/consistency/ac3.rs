use super::{AC3ErrorKind, Domain, State};
use crate::{Grid, WorkerBag};
use std::hash::Hash;

/// Runs the AC3 constraint satisfaction algorithm.
///
/// This restricts domains to only tiles consistent with the given constraint
///
/// Will return an error if:
/// - The initial tile restriction is not within the domain
/// - AC3 leads to a contradiction (an empty domain)
/// - Running the AC3 algorithm overflows a task buffer used
pub fn ac3<const N: usize, Idx>(
  grid: &(impl Grid<N, Idx> + Send + Sync),
  state: State<N, Idx>,
  domain_size: usize,
  start: &Idx,
  item: usize,
  constraint: impl (Fn(usize, usize, usize) -> bool) + Sync,
) -> Result<(), AC3ErrorKind>
where
  Idx: Hash + Eq + Clone + Send + Sync,
{
  let domain_hint = Domain::constraint(&constraint, domain_size);
  let workers: WorkerBag<(Idx, usize, usize)> = Default::default();

  if state.exists(start) && !state.read_at(start, |d| d.contains(item)).unwrap() {
    return Err(AC3ErrorKind::InvalidChoice);
  }

  state.or_insert_at(start, domain_hint.clone());
  let to_remove = state
    .write_at(start, |d| {
      std::mem::replace(d, Domain::single(item, domain_size))
        .into_iter()
        .filter(|&tile| tile != item)
    })
    .unwrap();

  workers.run_on(
    grid.updates_for(start, to_remove),
    move |(idx, side, tile)| {
      state.or_insert_at(&idx, domain_hint.clone());

      // calculate tiles to be removed from this neighbour
      // @note this will update the domain by removing those tiles
      let tiles_removed = (0..domain_size).filter(|&tile1| {
        constraint(tile, side, tile1)
          && state.write_at(&idx, |d| d.remove_side(tile, side)).unwrap()
      });

      (state.read_at(&idx, |d| !d.is_empty()).unwrap())
        .then_some(grid.updates_for(&idx, tiles_removed))
        .ok_or(AC3ErrorKind::InconsistentChoice)
    },
  )?;

  Ok(())
}
