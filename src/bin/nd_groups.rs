/// Unfortunately, we need to use some group theory for find out how to compare
/// tiles; we need to be able to find the minimal number of simple rotations to
/// bring any specific side to the "top" so that "faces" sit next to each other.
///
/// This one might take me a while to work out how to automate calculating how
/// rotations permute corners and all possible rotations for the sides. The
/// search itself should be solvable with a pretty straightforward dynamic
/// programming approach
///
/// found [this](https://www.gregegan.net/APPLETS/29/HypercubeNotes.html)
/// which suggests:
/// - #of rotations is (2^n n!)/2
///
/// For now, I'll just limit this to a manual approach for 2D and 3D
///
/// _.R - rolls the axis by one space, i.e. (x, y, z) -> (y, z, x)
/// [..;-1, .., ..] - flips the specified axes, i.e. flips x axis
///
/// 2D:
/// (Low,  Axis(0))(tile) = tile
/// (High, Axis(0))(tile) = tile[..;-1,..;-1]
/// (Low,  Axis(1))(tile) = tile.R
/// (High, Axis(1))(tile) = tile.R[..;-1,..;-1]
///
/// 3D:
/// (Low,  Axis(0))(tile) = tile
/// (High, Axis(0))(tile) = tile[..;-1,..;-1,..;-1]
/// (Low,  Axis(1))(tile) = tile.R
/// (High, Axis(1))(tile) = tile.R[..;-1,..;-1,..;-1]
/// (Low,  Axis(2))(tile) = tile.R.R
/// (High, Axis(2))(tile) = tile.R.R[..;-1,..;-1,..;-1]

fn corner_indices(n: usize) -> Vec<Vec<i8>> {
  if n == 0 {
    vec![vec![]]
  } else {
    vec![-1, 1]
      .into_iter()
      .flat_map(|i| {
        corner_indices(n - 1).into_iter().map(move |mut js| {
          js.push(i);
          js
        })
      })
      .collect()
  }
}

fn main() {
  println!("{:?}", corner_indices(3));
}
