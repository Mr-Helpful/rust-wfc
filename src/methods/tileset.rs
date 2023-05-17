// use ndarray::prelude::*;
// use rand::Rng;
// use std::{
//   collections::{hash_map::DefaultHasher, HashMap, HashSet},
//   hash::{Hash, Hasher},
// };

// type Img<T> = Array2<T>;

// #[derive(Hash, PartialEq, Eq, Clone)]
// enum Side {
//   North,
//   East,
//   South,
//   West,
// }

// impl Side {
//   fn fetch_edge<'a, T>(&'a self, img: &'a Img<T>) -> ArrayView1<'a, T> {
//     match *self {
//       Self::North => img.slice(s![0, ..]),
//       Self::East => img.slice(s![.., -1]),
//       Self::South => img.slice(s![-1, ..;-1]),
//       Self::West => img.slice(s![..;-1, 0]),
//     }
//   }
// }

// const ALL_SIDES: [Side; 4] = [Side::North, Side::East, Side::South, Side::West];

// type TileMatches = HashMap<(u64, Side), HashSet<(u64, Side)>>;

// fn rotated_constraints<T: Hash + Eq>(tiles: &Vec<(u64, Img<T>)>) -> TileMatches {
//   let mut matching: TileMatches = HashMap::new();

//   for (hash1, tile1) in tiles {
//     for side1 in ALL_SIDES {
//       matching.insert((*hash1, side1.clone()), HashSet::new());
//       let valid_set = matching.get_mut(&(*hash1, side1.clone())).unwrap();

//       for (hash2, tile2) in tiles {
//         for side2 in ALL_SIDES {
//           if side1
//             .fetch_edge(tile1)
//             .into_iter()
//             .rev()
//             .eq(side2.fetch_edge(tile2))
//           {
//             valid_set.insert((*hash2, side2));
//           }
//         }
//       }
//     }
//   }

//   matching
// }

// fn orthogonal_neighbours(
//   (w, h): (usize, usize),
//   (i, j): (usize, usize),
// ) -> impl Iterator<Item = (usize, usize)> {
//   [
//     (j > 0).then_some((i, j - 1)),
//     (i > 0).then_some((i - 1, j)),
//     (i < w - 1).then_some((i + 1, j)),
//     (j < h - 1).then_some((i, j + 1)),
//   ]
//   .into_iter()
//   .flatten()
// }

// type HashDomains = Array2<HashSet<(u64, Side)>>;

// fn try_arc3_resolve(
//   shape: (usize, usize),
//   mut domains: HashDomains,
//   constraint: TileMatches,
//   start: (usize, usize),
// ) -> Result<HashDomains, String> {
//   let mut to_check = vec![start];

//   while let Some(pos) = to_check.pop() {
//     let domain = domains.get(pos).unwrap().clone();

//     for neighbour_pos in orthogonal_neighbours(shape, pos) {
//       let neighbour = domains.get_mut(neighbour_pos).unwrap();
//       let prev_len = neighbour.len();
//       *neighbour = neighbour
//         .iter()
//         .filter(
//           |tile| !domain.is_disjoint(constraint.get(tile).unwrap()),
//           // i.e. intersection is non empty => some "supporting" tile is present
//         )
//         .cloned()
//         .collect();

//       if neighbour.is_empty() {
//         return Err("Assignment is not consistent".to_owned());
//       }
//       if prev_len != neighbour.len() {
//         to_check.push(neighbour_pos)
//       }
//     }
//   }

//   Ok(domains)
// }

// pub fn wfc_tiles<T: Hash + Eq, Rand: Rng>(
//   rng: &mut Rand,
//   (w, h): (usize, usize),
//   tiles: Vec<Img<T>>,
// ) {
//   assert!((w > 0) & (h > 0), "wx0 or 0xh dimensions not allowed");

//   let hash_tiles: Vec<(u64, Img<T>)> = tiles
//     .into_iter()
//     .map(|tile| {
//       let mut hasher = DefaultHasher::new();
//       tile.hash(&mut hasher);
//       (hasher.finish(), tile)
//     })
//     .collect();

//   let constraints = rotated_constraints(&hash_tiles);
//   let hash_to_tile: HashMap<u64, Img<T>> = HashMap::from_iter(hash_tiles);
//   let all_tiles = todo!();
//   let mut domains = Array2::from_elem((w, h), constraints.keys());
//   let history = vec![];

//   while domains.iter().any(|domain| domain.len() > 1) {
//     let (x, y) = (0..h)
//       .flat_map(|j| (0..w).map(|i| (i, j)))
//       .min_by_key(|&pos| domains.get(pos).unwrap().len())
//       .unwrap();

//     let domains = try_arc3_resolve((w, h), domains, constraints, (x, y));
//   }
// }
