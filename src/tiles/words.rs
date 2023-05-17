use super::{Direction, Tileable};

pub enum WordEnd {
  Prefix,
  Suffix,
}

impl Direction for WordEnd {
  fn opposite(&self) -> Self {
    match self {
      Self::Prefix => Self::Suffix,
      Self::Suffix => Self::Prefix,
    }
  }
}

pub struct Word(String);

impl Tileable<WordEnd> for Word {
  fn fits_together(&self, other: &Self, side: &WordEnd) -> bool {
    use WordEnd::*;
    let (first, second) = match side {
      Prefix => (other, self),
      Suffix => (self, other),
    };

    if let Some(last) = first.0.chars().last() {
      let mut init = "".to_owned();
      second.0.chars().any(|char| {
        init.push(char);
        char == last && first.0.ends_with(init.as_str())
      })
    } else {
      true
    }
  }
}
