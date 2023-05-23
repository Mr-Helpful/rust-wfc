mod space;
use space::Space;
mod domain;
pub use domain::Domain;
mod errors;
pub use errors::*;
mod constraint;
pub use constraint::*;

use crate::{Grid, WorkerBag};
use std::hash::Hash;

pub type CSPDomains<const N: usize, Idx> = Space<Idx, Domain<N>>;

/// Runs the AC3 constraint satisfaction algorithm.
///
/// This restricts domains to only tiles consistent with the given constraint
///
/// Will return an error if:
/// - The initial tile restriction is not within the domain
/// - AC3 leads to a contradiction (an empty domain)
/// - Running the AC3 algorithm overflows a task buffer used
pub fn ac3<const N: usize, Idx>(
  domains: CSPDomains<N, Idx>,
  domain_size: usize,
  grid: &(impl Grid<N, Idx> + Send + Sync),
  constraint: &Constraint<N>,
  start: &Idx,
  item: usize,
) -> Result<CSPDomains<N, Idx>, AC3ErrorKind>
where
  Idx: Hash + Eq + Clone + Send + Sync,
{
  let domain_hint = Domain::constraint(constraint, domain_size);
  let workers: WorkerBag<(Idx, usize, usize)> = Default::default();

  if domains.exists(start) && !domains.read_at(start, |d| d.contains(item)).unwrap() {
    return Err(AC3ErrorKind::InvalidChoice);
  }

  domains.or_insert_at(start, domain_hint.clone());
  let to_remove = domains
    .write_at(start, |d| {
      std::mem::replace(d, Domain::single(item, domain_size))
        .into_iter()
        .filter(|&tile| tile != item)
    })
    .unwrap();

  workers.run_on(grid.updates_for(start, to_remove), |(idx, side, tile)| {
    domains.or_insert_at(&idx, domain_hint.clone());

    // calculate tiles to be removed from this neighbour
    // @note this will update the domain by removing those tiles
    let tiles_removed = (0..domain_size).filter(|&tile1| {
      constraint[(tile, tile1, side)]
        && domains
          .write_at(&idx, |d| d.remove_side(tile, side))
          .unwrap()
    });

    (domains.read_at(&idx, |d| !d.is_empty()).unwrap())
      .then_some(grid.updates_for(&idx, tiles_removed))
      .ok_or(AC3ErrorKind::InconsistentChoice)
  })?;

  Ok(domains)
}
