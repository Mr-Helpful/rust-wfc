pub trait IterExtra: Iterator + Sized {
  fn count_where<P: Fn(&Self::Item) -> bool>(self, pred: P) -> usize {
    self.filter(pred).count()
  }
}
