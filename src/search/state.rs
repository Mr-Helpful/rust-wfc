use super::{State, WFCError};
use crate::{
  consistency::{ac3, AC3Error, CSPDomains, Constraint},
  grid::Grid,
  sampling::Sampler,
};
use std::hash::Hash;

/// A definition of state for the wfc algorithm.
///
/// Bundles together everything needed to assign a tile and propagate
/// constraints.
pub struct WFCState<'a, const N: usize, Idx, G, S> {
  /// A set of domains to assign to and constrain values within.
  domains: CSPDomains<N, Idx>,
  /// The maximum number of tiles that can be in any one domain.
  domain_size: usize,
  /// The grid used, informs which domains are constrained by each other.
  grid: &'a G,

  /// Picks the tile that should be assigned from a collection of tiles.
  /// This can either be deterministic or random, to allow for tile variation.
  pick_domain: S,
  /// A constraint on which tiles can be placed next to each other.
  constraint: &'a Constraint<N>,
}

impl<'a, const N: usize, Idx, G, S> WFCState<'a, N, Idx, G, S>
where
  Idx: Hash + Eq,
  G: Grid<N, Idx>,
{
  /// Helper method to calculate the AC3 heuristic for a domain
  fn ac3_heuristic(&self, idx: &Idx) -> [usize; 2] {
    let values: Vec<_> = self.domains.read_at(idx, |d| d.iter().collect()).unwrap();

    let mrv = values.len();
    if mrv == 0 {
      return [0, 0];
    }

    let degree = self
      .grid
      .neighbours(idx)
      .into_iter()
      .enumerate()
      .filter_map(|(side, optn)| {
        self.domains.read_at(&optn?, |d| {
          d.iter()
            .map(|tile1| {
              values
                .iter()
                .filter(|&&tile0| self.constraint[(tile0, tile1, side)])
                .count()
            })
            .sum::<usize>()
        })
      })
      .sum();

    [self.domain_size - mrv, degree]
  }
}

impl<'a, const N: usize, Idx, G, S> State for WFCState<'a, N, Idx, G, S>
where
  Idx: Clone + Hash + Eq + Send + Sync,
  G: Grid<N, Idx> + Send + Sync,
  S: Sampler + Clone,
{
  type Action = (Idx, usize);
  type Error = WFCError<Idx>;

  fn is_goal(&self) -> bool {
    self.domains.all(|d| d.is_single())
  }

  type ActnIter = Vec<(Idx, usize)>;
  fn get_actions(&self) -> Self::ActnIter {
    let idxs = self.domains.keys();
    let max_idx = idxs
      .into_iter()
      .max_by_key(|idx| self.ac3_heuristic(idx))
      .unwrap();

    self
      .domains
      .read_at(&max_idx, |d| {
        d.iter().map(|tile| (max_idx.clone(), tile)).collect()
      })
      .unwrap()
  }

  type PickError = WFCError<Idx>;
  fn pick_action<'b>(
    &'b mut self,
    actions: impl IntoIterator<Item = &'b Self::Action>,
  ) -> Result<Self::Action, Self::PickError> {
    let actions: Vec<_> = actions.into_iter().collect();
    if actions.is_empty() {
      return Err(WFCError::PickActionError("No actions available".to_owned()));
    }

    let (idx, _) = actions[0];
    let tiles: Vec<_> = actions.into_iter().map(|(_, tile)| *tile).collect();
    Ok((idx.clone(), self.pick_domain.sample(&tiles)))
  }

  type TakeError = AC3Error<Idx>;
  fn take_action(&self, (idx, tile): &Self::Action) -> Result<Self, Self::TakeError> {
    let domains = ac3(
      self.domains.clone(),
      self.domain_size,
      self.grid,
      self.constraint,
      idx,
      *tile,
    )
    .map_err(|kind| AC3Error::new(idx.clone(), *tile, kind))?;

    Ok(Self {
      domains,
      domain_size: self.domain_size,
      grid: self.grid,

      pick_domain: self.pick_domain.clone(),
      constraint: self.constraint,
    })
  }
}
