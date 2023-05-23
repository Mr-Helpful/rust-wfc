use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::RwLock;

/// An N dimensional space, intended to be used by multiple threads at once
#[derive(Debug)]
pub struct Space<Idx, T>(RwLock<HashMap<Idx, RwLock<T>>>);

impl<Idx: Clone + Hash + Eq, T: Clone> Clone for Space<Idx, T> {
  fn clone(&self) -> Self {
    Self(RwLock::new(
      self
        .0
        .read()
        .unwrap()
        .iter()
        .map(|(idx, rwlock)| (idx.clone(), RwLock::new(rwlock.read().unwrap().clone())))
        .collect(),
    ))
  }
}

impl<Idx, T> Default for Space<Idx, T> {
  fn default() -> Self {
    Self(RwLock::new(HashMap::new()))
  }
}

impl<Idx: Hash + Eq + Clone, T> Space<Idx, T> {
  /// Inserts an empty domain into a cell, if one doesn't exist
  pub fn or_insert_at(&self, idx: &Idx, value: T) {
    if !self.0.read().unwrap().contains_key(idx) {
      self
        .0
        .write()
        .expect("this is the only write we'll ever use, so we can guarantee no poisoned locks.")
        .insert(idx.clone(), RwLock::new(value));
    }
  }
}

impl<Idx: Clone, T> Space<Idx, T> {
  pub fn max_idx_by_key<O: Ord>(&self, rank: impl Fn(&Idx, &T) -> O) -> Option<Idx> {
    self
      .0
      .read()
      .unwrap()
      .iter()
      .filter_map(|(idx, lock)| lock.read().ok().map(|v| (idx, v)))
      .max_by_key(|(idx, v)| rank(idx, v))
      .map(|(idx, _)| idx.clone())
  }

  pub fn all(&self, pred: impl Fn(&T) -> bool) -> bool {
    self
      .0
      .read()
      .unwrap()
      .iter()
      .all(|(_, lock)| lock.read().map_or(false, |item| pred(&item)))
  }
}

impl<Idx: Clone, T> Space<Idx, T> {
  pub fn collect_key_func<I: IntoIterator>(
    &self,
    op: impl Fn(&T) -> I,
  ) -> Option<Vec<(Idx, I::Item)>> {
    let hashmap = self.0.read().ok()?;
    Some(
      hashmap
        .iter()
        .filter_map(|(idx, rwlock)| rwlock.read().ok().map(|item| (idx, item)))
        .flat_map(|(idx, item)| op(item.deref()).into_iter().map(|x| (idx.clone(), x)))
        .collect(),
    )
  }
}

impl<Idx: Hash + Eq, T> Space<Idx, T> {
  pub fn exists(&self, idx: &Idx) -> bool {
    self
      .0
      .read()
      .map_or(false, |hashmap| hashmap.contains_key(idx))
  }

  pub fn read_at<R>(&self, idx: &Idx, mut op: impl FnMut(&T) -> R) -> Option<R> {
    let hashmap = self.0.read().ok()?;
    let rwlock = hashmap.get(idx)?;
    let item = rwlock.read().ok()?;
    Some(op(&item))
  }

  pub fn write_at<R>(&self, idx: &Idx, mut op: impl FnMut(&mut T) -> R) -> Option<R> {
    let hashmap = self.0.read().ok()?;
    let rwlock = hashmap.get(idx)?;
    let mut item = rwlock.write().ok()?;
    Some(op(&mut item))
  }
}
