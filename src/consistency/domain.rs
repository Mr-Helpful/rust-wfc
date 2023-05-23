use crate::Constraint;
use std::array;

/// A set of indices that are currently valid for a cell
#[derive(Clone)]
pub struct Domain<const N: usize> {
  num_valid: usize,
  entries: Vec<(bool, [usize; N])>,
}

impl<const N: usize> Domain<N> {
  /// Generates a completely empty domain
  pub fn empty(size: usize) -> Self {
    let empty_entry = (false, [0; N]);
    Self {
      num_valid: 0,
      entries: (0..size).map(|_| empty_entry).collect(),
    }
  }

  /// Generates a domain containing a single item, with no info on side counts
  pub fn single(i: usize, size: usize) -> Self {
    let mut domain = Self::empty(size);
    domain.entries[i].0 = true;
    domain.num_valid = 1;
    domain
  }

  /// Initialises a domain from a constraint on the values taken per side
  pub fn constraint(constraint: &Constraint<N>, no_tiles: usize) -> Self {
    let entries: Vec<_> = (0..no_tiles)
      .map(|tile0| {
        let side_counts = array::from_fn(|side| {
          (0..no_tiles)
            .filter(|&tile1| constraint[(tile0, tile1, side)])
            .count()
        });

        (side_counts.iter().any(|&count| count > 0), side_counts)
      })
      .collect();

    Domain {
      num_valid: entries.iter().filter(|entry| entry.0).count(),
      entries,
    }
  }
}

impl<const N: usize> Domain<N> {
  /// Whether there are no valid items left in the domain
  pub fn is_empty(&self) -> bool {
    self.num_valid == 0
  }

  /// Whether there is a single valid item left in the domain
  pub fn is_single(&self) -> bool {
    self.num_valid == 1
  }

  /// Whether the domain contains the given item
  pub fn contains(&self, item: usize) -> bool {
    self.entries.get(item).map_or(false, |entry| entry.0)
  }

  /// Removes an item from this domain
  pub fn remove_item(&mut self, item: usize) -> bool {
    if !self.contains(item) {
      return false;
    }
    self.entries[item].0 = false;
    self.num_valid -= 1;
    true
  }

  /// Removes an item from a given side of the domain, returning whether an
  /// entry was actually removed due to this
  pub fn remove_side(&mut self, item: usize, side: usize) -> bool {
    if self.num_valid == 0 || !self.contains(item) {
      return false;
    }

    let entry = &mut self.entries[item];
    entry.1[side] -= 1;
    if entry.1[side] > 0 {
      return false;
    }

    entry.0 = false;
    self.num_valid -= 1;
    true
  }

  pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
    self
      .entries
      .iter()
      .enumerate()
      .filter_map(|(i, entry)| entry.0.then_some(i))
  }
}

impl<const N: usize> IntoIterator for Domain<N> {
  type Item = usize;
  type IntoIter = DomainIter<N>;
  fn into_iter(self) -> Self::IntoIter {
    DomainIter {
      i: 0,
      count: 0,
      domain: self,
    }
  }
}

#[derive(Clone)]
pub struct DomainIter<const N: usize> {
  i: usize,
  count: usize,
  domain: Domain<N>,
}

impl<const N: usize> Iterator for DomainIter<N> {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
    if self.count == self.domain.num_valid {
      return None;
    }
    if self.domain.entries[self.i].0 {
      self.i += 1;
      self.count += 1;
      return Some(self.i - 1);
    }
    self.i += 1;
    self.next()
  }
}
