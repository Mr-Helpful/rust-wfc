use std::array::from_fn;

#[derive(Clone, Copy)]
pub struct DomainEntry<const N: usize> {
  pub(crate) valid: bool,
  pub(crate) side_counts: [usize; N],
}

#[derive(Clone, Default)]
pub struct Domain<const N: usize> {
  pub(crate) no_valid: usize,
  pub(crate) entries: Vec<DomainEntry<N>>,
}

impl<const N: usize> Domain<N> {
  /// Generates a completely empty domain
  pub fn empty(size: usize) -> Self {
    let empty_entry = DomainEntry {
      valid: false,
      side_counts: [0; N],
    };
    Self {
      no_valid: 0,
      entries: (0..size).map(|_| empty_entry).collect(),
    }
  }

  /// Generates a completely filled domain, i.e. all items present
  pub fn full(size: usize) -> Self {
    let full_entry = DomainEntry {
      valid: true,
      side_counts: [0; N],
    };
    Self {
      no_valid: size,
      entries: (0..size).map(|_| full_entry).collect(),
    }
  }

  /// Generates a domain containing a single item, with no info on side counts
  pub fn single(i: usize, size: usize) -> Self {
    let mut domain = Self::empty(size);
    domain.entries[i].valid = true;
    domain.no_valid = 1;
    domain
  }

  /// Initialises a domain from a constraint on the values taken per side
  pub fn from_constraint<C: Fn(usize, usize, usize) -> bool>(constraint: C, size: usize) -> Self {
    let entries: Vec<_> = (0..size)
      .map(|tile0| {
        let side_counts = from_fn(|side| {
          (0..size)
            .filter(|&tile1| constraint(tile0, side, tile1))
            .count()
        });

        DomainEntry {
          valid: side_counts.iter().any(|&count| count > 0),
          side_counts,
        }
      })
      .collect();

    Domain {
      no_valid: entries.iter().filter(|entry| entry.valid).count(),
      entries,
    }
  }

  pub fn has(&self, item: usize) -> bool {
    (0..self.entries.len()).contains(&item) && self.entries[item].valid
  }

  /// Removes an item from this domain, returning whether an entry was
  /// removed due to this
  pub fn remove_item(&mut self, item: usize) -> bool {
    if !self.has(item) {
      return false;
    }
    self.entries[item].valid = false;
    self.no_valid -= 1;
    true
  }

  /// Removes an item from a given side of the domain, returning whether an
  /// entry was actually removed due to this
  pub fn remove_side(&mut self, item: usize, side: usize) -> bool {
    if self.no_valid == 0 || !self.has(item) {
      return false;
    }

    let entry = &mut self.entries[item];
    entry.side_counts[side] -= 1;
    if entry.side_counts[side] > 0 {
      return false;
    }

    entry.valid = false;
    self.no_valid -= 1;
    true
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

pub struct DomainIter<const N: usize> {
  i: usize,
  count: usize,
  domain: Domain<N>,
}

impl<const N: usize> Iterator for DomainIter<N> {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
    if self.count == self.domain.no_valid {
      return None;
    }
    if self.domain.entries[self.i].valid {
      self.i += 1;
      self.count += 1;
      return Some(self.i - 1);
    }
    self.i += 1;
    self.next()
  }
}
