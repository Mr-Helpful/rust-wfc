use std::collections::HashMap;
use std::hash::Hash;
use std::ops::DerefMut;
use std::sync::RwLock;

use super::{Domain, WorkerBag};
use crate::Grid;

pub struct State<const N: usize, Idx, G: Grid<N, Idx>> {
  grid: G,

  /// the possibilities for each position denoted by `Idx`
  /// we use two locks here:
  /// - the inner lock for modifying values in place
  /// - the outer lock for inserting default values in the map
  domains: RwLock<HashMap<Idx, RwLock<Domain<N>>>>,

  /// the number of seperate tiles that are in a domain
  domain_size: usize,

  /// a hint on how to initialise the side counts of a new domain<br>
  /// this can save a tiny bit of computation for arc3 consistency
  domain_hint: Option<Domain<N>>,

  worker_bag: WorkerBag<(Idx, usize, usize)>,
}

impl<const N: usize, Idx, G> State<N, Idx, G>
where
  Idx: Hash + Eq + Clone,
  G: Grid<N, Idx>,
{
  pub fn new<T>(grid: G, items: &[T]) -> Self {
    Self {
      grid,
      domains: RwLock::new(HashMap::new()),
      domain_size: items.len(),
      domain_hint: None,
      worker_bag: Default::default(),
    }
  }

  /// Either initialises from the domain hint or a full domain, if a hint
  /// isn't present.
  fn domain_from_hint(&self) -> Domain<N> {
    self
      .domain_hint
      .clone()
      .unwrap_or_else(|| Domain::full(self.domain_size))
  }

  /// Tests whether a particular item is in a particular domain
  fn has_domain_item(&self, idx: &Idx, item: usize) -> bool {
    self
      .domains
      .read()
      .unwrap()
      .get(idx)
      .map_or(false, |domain| domain.read().unwrap().has(item))
  }

  /// Inserts an empty domain into a cell, if one doesn't exist
  fn insert_default_domain(&self, idx: &Idx) {
    if !self.domains.read().unwrap().contains_key(idx) {
      self
        .domains
        .write()
        .expect("this is the only write we'll ever use, so we can guarantee no poisoned locks.")
        .insert(idx.clone(), RwLock::new(self.domain_from_hint()));
    }
  }

  /// Swaps the domain at `idx` with a single value domain and returns the
  /// old domain
  fn swap_single(&mut self, idx: &Idx, item: usize) -> Domain<N> {
    self.insert_default_domain(idx);
    std::mem::replace(
      self
        .domains
        .read()
        .unwrap()
        .get(idx)
        .expect("we'll have filled the domain via the call to `insert_default_domain` here")
        .write()
        .unwrap()
        .deref_mut(),
      Domain::single(item, self.domain_size),
    )
  }

  /// Adds a hint for how to initialise domains based a given constraint
  fn add_domain_hint(&mut self, constraint: impl Fn(usize, usize, usize) -> bool) {
    if self.domain_hint.is_none() {
      self.domain_hint = Some(Domain::from_constraint(constraint, self.domain_size))
    }
  }
}

pub use super::{AC3Error, AC3ErrorKind};

impl<const N: usize, Idx, G> State<N, Idx, G>
where
  Idx: Hash + Eq + Clone + Send + Sync,
  G: Grid<N, Idx> + Sync,
{
  /// Determines the neighbouring cells that need to be checked to make ensure
  /// the tile removal remains arc consistent
  fn updates_needed(
    &self,
    idx: &Idx,
    to_remove: impl IntoIterator<Item = usize>,
  ) -> impl Iterator<Item = (Idx, usize, usize)> {
    let sides = self
      .grid
      .neighbours(idx)
      .into_iter()
      .enumerate()
      .filter_map(|(side, optn)| optn.map(|idx| (idx, side)));
    to_remove
      .into_iter()
      .flat_map(move |tile| sides.clone().map(move |(idx, side)| (idx, side, tile)))
  }

  /// Makes a single cell arc consistent with a given tile removal
  /// and returns updates for all tiles removed to its neighbours
  fn make_arc_consistent(
    &self,
    constraint: impl Fn(usize, usize, usize) -> bool + Sync,
    (idx, tile, side): (Idx, usize, usize),
  ) -> Result<Vec<(Idx, usize, usize)>, AC3ErrorKind> {
    self.insert_default_domain(&idx);
    let domains = &self.domains.read().unwrap();
    let mut domain = domains.get(&idx).unwrap().write().unwrap();

    // calculate tiles to be removed from this neighbour
    // @note this will update the domain by removing unsupported tiles
    let to_remove = (0..self.domain_size)
      .filter(|&tile1| constraint(tile, side, tile1) && domain.remove_side(tile, side));
    let updates: Vec<_> = self.updates_needed(&idx, to_remove).collect();

    (domain.no_valid > 0)
      .then_some(updates)
      .ok_or(AC3ErrorKind::InconsistentChoice)
  }

  pub fn run_ac3(
    &mut self,
    start: &Idx,
    item: usize,
    constraint: impl (Fn(usize, usize, usize) -> bool) + Sync,
  ) -> Result<(), AC3Error<Idx>> {
    self.add_domain_hint(&constraint);
    self.insert_default_domain(start);
    if self.has_domain_item(start, item) {
      self.domain_hint = None;
      return Err(AC3Error::new(
        start.clone(),
        item,
        AC3ErrorKind::InvalidChoice,
      ));
    }

    let mut domain = self.swap_single(start, item);
    domain.remove_item(item);
    self
      .worker_bag
      .run_on(
        |(idx, side, tile)| self.make_arc_consistent(&constraint, (idx, side, tile)),
        self.updates_needed(start, domain),
      )
      .map_err(|err| AC3Error::new(start.clone(), item, err))?;

    self.domain_hint = None;
    Ok(())
  }
}
