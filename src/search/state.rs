use crate::{ac3, AC3Error, CSPDomains, Constraint, Domain, Grid, Sampler, State};
use std::{collections::HashMap, hash::Hash};

pub struct CSPState<'a, const N: usize, Idx, O, G, H, S>
where
  O: Ord,
  G: Grid<N, Idx>,
  H: Fn(&Idx, &Domain<N>) -> O,
  S: Sampler,
{
  domains: CSPDomains<N, Idx>,
  domain_size: usize,
  grid: &'a G,

  rank_domain: &'a H,
  pick_domain: S,
  constraint: &'a Constraint<N>,
}

impl<'a, const N: usize, Idx, O, G, H, S> State for CSPState<'a, N, Idx, O, G, H, S>
where
  O: Ord,
  Idx: Clone + Hash + Eq + Send + Sync,
  G: Grid<N, Idx> + Send + Sync,
  H: Fn(&Idx, &Domain<N>) -> O,
  S: Sampler + Clone,
{
  type Action = (Idx, usize);
  type Error = AC3Error<Idx>;

  fn is_goal(&self) -> bool {
    self.domains.all(|d| d.is_single())
  }

  type ActnIter = Vec<(Idx, usize)>;
  fn get_actions(&self) -> Self::ActnIter {
    self
      .domains
      .collect_key_func(|d| d.iter().collect::<Vec<_>>())
      .expect("We should be able to access the domains")
  }

  type PickError = AC3Error<Idx>; // not currently using this
  fn pick_action<'b>(
    &'b mut self,
    actions: impl IntoIterator<Item = &'b Self::Action>,
  ) -> Result<Self::Action, Self::PickError> {
    let mut idx_map: HashMap<Idx, Vec<usize>> = Default::default();
    for (idx, tile) in actions {
      idx_map.entry(idx.clone()).or_default().push(*tile);
    }

    let (idx, tiles) = idx_map
      .into_iter()
      .max_by_key(|(idx, _)| self.domains.read_at(idx, |d| (self.rank_domain)(idx, d)))
      .unwrap();

    Ok((idx, self.pick_domain.sample(&tiles)))
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

      rank_domain: self.rank_domain,
      pick_domain: self.pick_domain.clone(),
      constraint: self.constraint,
    })
  }
}
