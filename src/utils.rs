pub trait FromFnCount<T>: FromIterator<T> {
  fn from_fn_count(count: usize, f: fn(usize) -> T) -> Self {
    (0..count).map(f).collect()
  }
}

pub trait IterExtra: Iterator + Sized {
  fn count_where<P: Fn(&Self::Item) -> bool>(self, pred: P) -> usize {
    self.filter(pred).count()
  }
}
