use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::RwLock;

pub struct Space<Idx, T>(RwLock<HashMap<Idx, RwLock<T>>>);

impl<Idx, T> Deref for Space<Idx, T> {
  type Target = RwLock<HashMap<Idx, RwLock<T>>>;
  fn deref(&self) -> &Self::Target {
    &self.0
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
        .write()
        .expect("this is the only write we'll ever use, so we can guarantee no poisoned locks.")
        .insert(idx.clone(), RwLock::new(value));
    }
  }
}

impl<Idx: Hash + Eq, T> Space<Idx, T> {
  pub fn exists(&self, idx: &Idx) -> bool {
    self
      .0
      .read()
      .map_or(false, |hashmap| hashmap.contains_key(idx))
  }

  pub fn read_at<R>(&self, idx: &Idx, op: impl Fn(&T) -> R) -> Option<R> {
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