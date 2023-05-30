use super::{Direction, Tileable};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum WordSide {
  Prefix,
  #[default]
  Suffix,
}

impl Direction for WordSide {
  fn opposite(&self) -> Self {
    match self {
      Self::Prefix => Self::Suffix,
      Self::Suffix => Self::Prefix,
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Word(String);

impl Tileable<WordSide> for Word {
  fn tiles(&self, other: &Self, side: &WordSide) -> bool {
    use WordSide::*;
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
