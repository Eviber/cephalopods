use itertools::Itertools;

use std::{collections::HashMap, io};

fn get_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

type Bitset = u32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Grid {
    bitset: Bitset,
}

impl Grid {
    #[inline]
    fn get(&self, x: usize, y: usize) -> u32 {
        (self.bitset >> ((y * 3 + x) * 3)) & 7
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, n: u32) {
        self.bitset &= !(7 << ((y * 3 + x) * 3));
        self.bitset |= n << ((y * 3 + x) * 3);
    }

    fn from_input() -> Grid {
        let mut grid = Grid::default();
        for y in 0..3 {
            let inputs = get_line();
            for (x, s) in inputs.split_whitespace().enumerate() {
                let value: u32 = s.parse().unwrap();
                grid.set(x, y, value);
            }
        }
        grid
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..3 {
            for x in 0..3 {
                eprint!("{}", self.get(x, y));
            }
            eprintln!();
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn empty_mask(&self) -> Bitset {
        !self.occupied() & 0x1FF
    }

    #[inline]
    fn occupied(&self) -> Bitset {
        let mut set = 0;
        for y in 0..3 {
            for x in 0..3 {
                set <<= 1;
                if self.get(x, y) != 0 {
                    set |= 1;
                }
            }
        }
        set
    }

    fn possible_states(&self) -> Vec<Grid> {
        let mut possible_states = Vec::new();

        for y in 0..3 {
            for x in 0..3 {
                if self.get(x, y) != 0 {
                    continue;
                }
                let mut neighbours = Vec::with_capacity(4);
                if y > 0 && self.get(x, y - 1) != 0 && self.get(x, y - 1) != 6 {
                    neighbours.push((x, y - 1));
                }
                if x > 0 && self.get(x - 1, y) != 0 && self.get(x - 1, y) != 6 {
                    neighbours.push((x - 1, y));
                }
                if y < 2 && self.get(x, y + 1) != 0 && self.get(x, y + 1) != 6 {
                    neighbours.push((x, y + 1));
                }
                if x < 2 && self.get(x + 1, y) != 0 && self.get(x + 1, y) != 6 {
                    neighbours.push((x + 1, y));
                }
                let mut valid_neighbours = false;
                for comb in neighbours
                    .iter()
                    .combinations(4)
                    .chain(neighbours.iter().combinations(3))
                    .chain(neighbours.iter().combinations(2))
                {
                    let mut sum = 0;
                    for &&(x, y) in &comb {
                        sum += self.get(x, y);
                    }
                    if sum > 6 {
                        continue;
                    }
                    valid_neighbours = true;
                    let mut new_state = *self;
                    for &(x, y) in comb {
                        new_state.set(x, y, 0);
                    }
                    new_state.set(x, y, sum);
                    possible_states.push(new_state);
                }
                if !valid_neighbours {
                    let mut new_state = *self;
                    new_state.set(x, y, 1);
                    possible_states.push(new_state);
                }
            }
        }
        possible_states
    }

    #[inline]
    fn hash(&self) -> u32 {
        let mut hash = 0;

        for n in 0..9 {
            hash = hash * 10 + self.get(n % 3, n / 3);
        }
        hash
    }

    #[inline]
    fn dice_count(&self) -> usize {
        self.occupied().count_ones() as usize
    }

    #[inline]
    fn h_flip(&self) -> Self {
        Grid {
            bitset: ((self.bitset >> (3 * 6)) & 0o777)
                | ((self.bitset & 0o777) << (3 * 6))
                | (self.bitset & 0o777_000),
        }
    }

    #[inline]
    fn v_flip(&self) -> Self {
        Grid {
            bitset: ((self.bitset >> (2 * 3)) & 0o007_007_007)
                | ((self.bitset & 0o007_007_007) << (2 * 3))
                | (self.bitset & 0o070_070_070),
        }
    }

    #[inline]
    fn vh_flip(&self) -> Self {
        self.h_flip().v_flip()
    }

    // 1 2 3 4 5 6 7 8 9
    //   > | > | < | <
    // 9 6 3 8 5 2 7 4 1

    // 1 2 3 4 5 6 7 8 9
    // | ) > ( | ) < ( |
    // 1 4 7 2 5 8 3 6 9
    #[inline]
    fn d_flip(&self) -> Self {
        Grid {
            bitset: self.bitset & 0o_700_070_007
                | ((self.bitset & 0o_070_007_000) >> (2 * 3))
                | ((self.bitset & 0o_007_000_000) >> (4 * 3))
                | ((self.bitset & 0o_000_700_070) << (2 * 3))
                | ((self.bitset & 0o_000_000_700) << (4 * 3)),
        }
    }

    #[inline]
    fn dv_flip(&self) -> Self {
        self.d_flip().v_flip()
    }

    #[inline]
    fn dh_flip(&self) -> Self {
        self.d_flip().h_flip()
    }

    #[inline]
    fn dvh_flip(&self) -> Self {
        // bitset: self.bitset & 0o_007_070_700
        //     | ((self.bitset & 0o_070_700_000) >> (4 * 3))
        //     | ((self.bitset & 0o_000_007_070) << (4 * 3))
        //     | ((self.bitset & 0o_000_000_007) << (8 * 3))
        //     | ((self.bitset & 0o_700_000_000) >> (8 * 3)),
        self.d_flip().v_flip().h_flip()
    }

    #[inline]
    fn canonical(&self) -> (usize, Self) {
        [
            *self,
            self.v_flip(),
            self.h_flip(),
            self.vh_flip(),
            self.d_flip(),
            self.dv_flip(),
            self.dh_flip(),
            self.dvh_flip(),
        ]
        .iter()
        .copied()
        .enumerate()
        .min_by_key(|(_, v)| v.bitset)
        .unwrap()
    }
}

// Array containing every current and future states.
// first is the total sum of every dies (len 2, for current and next)
// second dimension is the number of dies (len 9, considering the 0 case doesn't exist)
//
// it is only possible to move from one state to another by:
// adding a dice and incrementing the sum
// or by removing 1 to 3 dies and keeping the sum (capture, so minus 2 to 4 plus 1)
//
// every cell contains a HashMap with grid state as key and an array Paths as value
// Paths contains how many ways there are to attain this grid state, by depth
// ex: [4, 3, 0, 1, ..] would mean 4 ways to attain this state at depth 0, 3 at depth 1, etc.
//
// First initialize the array with empty maps everywhere
// Then insert the starting state
//
// In a loop:
//
//   Iterate over the array, from the least sum & most dies to the least sum & least dies
//   for every encountered state, generate every children states and add the current Paths to the
//   children, after decrementing the remaining depth
//
//   Everytime a depth of zero is reached, add the hash of the grid times the number of
//   ways it was reached to the total hash.
//   If a grid reaches 9 dies, add its hash times the sum of all its Paths to the total hash.
//
//   Then, clear the current array and swap next and current arrays
//
//   if every HashMap was empty this iteration, break out of the loop
//
// end of loop

const MAX_DEPTH: usize = 40;
const TRANSFORM_COUNT: usize = 8;
type Paths = [[u32; MAX_DEPTH + 1]; TRANSFORM_COUNT];
type GridStateMap = HashMap<Grid, Paths>;

struct StateBuffer([Option<GridStateMap>; 9]);

impl StateBuffer {
    fn new(grid: Grid, depth: usize) -> Self {
        let mut state_buffer = StateBuffer([
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
        ]);
        let mut paths = [[0; MAX_DEPTH + 1]; TRANSFORM_COUNT];
        paths[0][depth] = 1; // Not sure about that 0...
        let idx = if grid.hash() == 0 {
            7 // we can put an initial empty grid anywhere except at index 0 or 8
        } else {
            9 - grid.dice_count()
        };
        state_buffer.insert(idx, grid, paths);
        state_buffer
    }

    #[inline]
    fn insert(&mut self, idx: usize, grid: Grid, paths: Paths) {
        self.0[idx].as_mut().unwrap().insert(grid, paths);
    }

    #[inline]
    fn is_empty(&self, idx: usize) -> bool {
        self.0[idx].as_ref().unwrap().is_empty()
    }

    #[inline]
    fn take(&mut self, idx: usize) -> GridStateMap {
        self.0[idx].take().unwrap()
    }

    #[inline]
    fn entry(&mut self, idx: usize, grid: Grid) -> &mut Paths {
        self.0[idx]
            .as_mut()
            .unwrap()
            .entry(grid)
            .or_insert_with(|| [[0; MAX_DEPTH + 1]; TRANSFORM_COUNT])
    }

    #[inline]
    fn put_back(&mut self, idx: usize, mut grid_states: GridStateMap) {
        grid_states.clear();
        self.0[idx] = Some(grid_states);
    }

    // Pour :
    // 0 - Identité
    // 1 - Vertical
    // 2 - Horizontal
    // 3 - VH
    // 4 - Diagonal
    // 5 - DV
    // 6 - DH
    // 7 - DVH
    //
    // avec f(a, b) = f(8a + b)
    // f_table = [
    //     0, 1, 2, 3, 4, 5, 6, 7,
    //     1, 0, 3, 2, 6, 7, 4, 5,
    //     2, 3, 0, 1, 5, 4, 7, 6,
    //     3, 2, 1, 0, 7, 6, 5, 4,
    //     4, 5, 6, 7, 0, 1, 2, 3,
    //     5, 4, 7, 6, 2, 3, 0, 1,
    //     6, 7, 4, 5, 1, 0, 3, 2,
    //     7, 6, 5, 4, 3, 2, 1, 0
    // ]
    #[inline]
    fn add_grid(&mut self, grid: Grid, parent_path: &Paths) {
        let next_dice_count = grid.dice_count();
        let (transform_idx, canon_transform) = grid.canonical();
        let f_table = [
            [0, 1, 2, 3, 4, 5, 6, 7],
            [1, 0, 3, 2, 5, 4, 7, 6],
            [2, 3, 0, 1, 6, 7, 4, 5],
            [3, 2, 1, 0, 7, 6, 5, 4],
            [4, 6, 5, 7, 0, 2, 1, 3],
            [5, 7, 4, 6, 1, 3, 0, 2],
            [6, 4, 7, 5, 2, 0, 3, 1],
            [7, 5, 6, 4, 3, 1, 2, 0],
        ];
        let p = self.entry(9 - next_dice_count, canon_transform);
        for (t_before, &t_after) in f_table[transform_idx].iter().enumerate() {
            for i in 0..MAX_DEPTH {
                p[t_after][i] += parent_path[t_before][i + 1];
            }
        }
    }
}

fn compute_sum(grid: Grid, depth: usize) -> u32 {
    let mut final_sum = 0;
    let mut state_buffer = StateBuffer::new(grid, depth);

    loop {
        let mut was_empty = true;
        for i in 0..9 {
            if state_buffer.is_empty(i) {
                continue;
            }
            let grid_states = state_buffer.take(i);
            was_empty = false;
            for (grid, path) in &grid_states {
                if i == 0 {
                    final_sum += path[0].iter().sum::<u32>() * grid.hash();
                    final_sum += path[1].iter().sum::<u32>() * grid.v_flip().hash();
                    final_sum += path[2].iter().sum::<u32>() * grid.h_flip().hash();
                    final_sum += path[3].iter().sum::<u32>() * grid.vh_flip().hash();
                    final_sum += path[4].iter().sum::<u32>() * grid.d_flip().hash();
                    final_sum += path[5].iter().sum::<u32>() * grid.dh_flip().hash();
                    final_sum += path[6].iter().sum::<u32>() * grid.dv_flip().hash();
                    final_sum += path[7].iter().sum::<u32>() * grid.dvh_flip().hash();
                    continue;
                }
                final_sum += path[0][0] * grid.hash();
                final_sum += path[1][0] * grid.v_flip().hash();
                final_sum += path[2][0] * grid.h_flip().hash();
                final_sum += path[3][0] * grid.vh_flip().hash();
                final_sum += path[4][0] * grid.d_flip().hash();
                final_sum += path[5][0] * grid.dh_flip().hash();
                final_sum += path[6][0] * grid.dv_flip().hash();
                final_sum += path[7][0] * grid.dvh_flip().hash();

                if path
                    .iter()
                    .flat_map(|depths| depths.iter().skip(1))
                    .all(|&n| n == 0)
                {
                    continue;
                }
                for g in grid.possible_states() {
                    state_buffer.add_grid(g, path);
                }
            }
            state_buffer.put_back(i, grid_states);
        }
        if was_empty {
            break;
        }
    }
    for m in state_buffer.0 {
        let m = m.unwrap();
        eprintln!("{} - {}", m.len(), m.capacity());
    }
    eprintln!();
    final_sum % (1 << 30)
}

fn main() {
    let depth: usize = get_line().trim().parse().unwrap();
    let grid = Grid::from_input();
    grid.print();
    eprintln!();

    let final_sum = compute_sum(grid, depth);
    println!("{final_sum}");
}

#[allow(dead_code)]
fn print_paths(p: &Paths) {
    eprint!("P:");
    for (t_i, t) in p.iter().enumerate() {
        for (d_i, &d) in t.iter().enumerate() {
            if d != 0 {
                eprint!(" - t:{}, ", t_i);
                eprint!("d:{} ", d_i);
                eprint!("=> {}", d);
            }
        }
    }
    eprintln!();
}

#[cfg(test)]
mod tests {
    use super::*;
    fn grid_from_array(a: [u32; 9]) -> Grid {
        let mut g = Grid { bitset: 0 };
        for (i, n) in a.into_iter().enumerate() {
            g.set(i % 3, i / 3, n);
        }
        g
    }

    #[test]
    fn test_1() {
        let depth = 20;
        let grid = grid_from_array([0, 6, 0, 2, 2, 2, 1, 6, 1]);
        assert_eq!(compute_sum(grid, depth), 322444322);
    }

    #[test]
    fn test_2() {
        let depth = 20;
        let grid = grid_from_array([5, 0, 6, 4, 5, 0, 0, 6, 4]);
        assert_eq!(compute_sum(grid, depth), 951223336);
    }

    #[test]
    fn test_3() {
        let depth = 1;
        let grid = grid_from_array([5, 5, 5, 0, 0, 5, 5, 5, 5]);
        assert_eq!(compute_sum(grid, depth), 36379286);
    }

    #[test]
    fn test_4() {
        let depth = 1;
        let grid = grid_from_array([6, 1, 6, 1, 0, 1, 6, 1, 6]);
        assert_eq!(compute_sum(grid, depth), 264239762);
    }

    #[test]
    fn test_5() {
        let depth = 8;
        let grid = grid_from_array([6, 0, 6, 0, 0, 0, 6, 1, 5]);
        assert_eq!(compute_sum(grid, depth), 76092874);
    }

    #[test]
    fn test_6() {
        let depth = 24;
        let grid = grid_from_array([3, 0, 0, 3, 6, 2, 1, 0, 2]);
        assert_eq!(compute_sum(grid, depth), 661168294);
    }

    #[test]
    fn test_7() {
        let depth = 36;
        let grid = grid_from_array([6, 0, 4, 2, 0, 2, 4, 0, 0]);
        assert_eq!(compute_sum(grid, depth), 350917228);
    }

    #[test]
    fn test_8() {
        let depth = 32;
        let grid = grid_from_array([0, 0, 0, 0, 5, 4, 1, 0, 5]);
        assert_eq!(compute_sum(grid, depth), 999653138);
    }

    #[test]
    fn test_9() {
        let depth = 40;
        let grid = grid_from_array([0, 0, 4, 0, 2, 4, 1, 3, 4]);
        assert_eq!(compute_sum(grid, depth), 521112022);
    }

    #[test]
    fn test_10() {
        let depth = 40;
        let grid = grid_from_array([0, 5, 4, 0, 3, 0, 0, 3, 0]);
        assert_eq!(compute_sum(grid, depth), 667094338);
    }

    #[test]
    fn test_11() {
        let depth = 20;
        let grid = grid_from_array([0, 5, 1, 0, 0, 0, 4, 0, 1]);
        assert_eq!(compute_sum(grid, depth), 738691369);
    }

    #[test]
    fn test_12() {
        let depth = 20;
        let grid = grid_from_array([1, 0, 0, 3, 5, 2, 1, 0, 0]);
        assert_eq!(compute_sum(grid, depth), 808014757);
    }

    // Pour :
    // 0 - Identité
    // 1 - Vertical
    // 2 - Horizontal
    // 3 - VH
    // 4 - Diagonal
    // 5 - DV
    // 6 - DH
    // 7 - DVH
    //
    // avec f(a, b) = f(8a + b)
    // f_table = [
    //     0, 1, 2, 3, 4, 5, 6, 7,
    //     1, 0, 3, 2, 6, 7, 4, 5,
    //     2, 3, 0, 1, 5, 4, 7, 6,
    //     3, 2, 1, 0, 7, 6, 5, 4,
    //     4, 5, 6, 7, 0, 1, 2, 3,
    //     5, 4, 7, 6, 2, 3, 0, 1,
    //     6, 7, 4, 5, 1, 0, 3, 2,
    //     7, 6, 5, 4, 3, 2, 1, 0
    // ]
    #[test]
    fn test_transforms_diag() {
        let grid = grid_from_array([0, 1, 2, 3, 4, 5, 6, 0, 1]);
        assert_eq!(grid.d_flip().v_flip(), grid.dv_flip());
        assert_eq!(grid.d_flip().h_flip(), grid.dh_flip());
        assert_eq!(grid.d_flip().vh_flip(), grid.dvh_flip());
        assert_eq!(grid.d_flip().d_flip(), grid);
        assert_eq!(grid.d_flip().dv_flip(), grid.v_flip());
        assert_eq!(grid.d_flip().dh_flip(), grid.h_flip());
        assert_eq!(grid.d_flip().dvh_flip(), grid.vh_flip());

        assert_eq!(grid.dv_flip().d_flip(), grid.h_flip());
        assert_eq!(grid.dv_flip().dv_flip(), grid.vh_flip());
        assert_eq!(grid.dv_flip().dvh_flip(), grid.v_flip());
    }

    #[test]
    fn test_transforms_table() {
        let grid = grid_from_array([0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let f_table = [
            [0, 1, 2, 3, 4, 5, 6, 7],
            [1, 0, 3, 2, 6, 7, 4, 5],
            [2, 3, 0, 1, 5, 4, 7, 6],
            [3, 2, 1, 0, 7, 6, 5, 4],
            [4, 5, 6, 7, 0, 1, 2, 3],
            [5, 4, 7, 6, 2, 3, 0, 1],
            [6, 7, 4, 5, 1, 0, 3, 2],
            [7, 6, 5, 4, 3, 2, 1, 0],
        ];

        let transforms = [
            grid,
            grid.v_flip(),
            grid.h_flip(),
            grid.vh_flip(),
            grid.d_flip(),
            grid.dv_flip(),
            grid.dh_flip(),
            grid.dvh_flip(),
        ];

        for (grid_base, list) in transforms.into_iter().zip(f_table.into_iter()) {
            for (i, grid_output) in list.into_iter().map(|i| transforms[i]).enumerate() {
                let transform = match i {
                    0 => grid_base,
                    1 => grid_base.v_flip(),
                    2 => grid_base.h_flip(),
                    3 => grid_base.vh_flip(),
                    4 => grid_base.d_flip(),
                    5 => grid_base.dv_flip(),
                    6 => grid_base.dh_flip(),
                    7 => grid_base.dvh_flip(),
                    _ => unreachable!(),
                };
                assert_eq!(transform, grid_output);
            }
        }
    }
}
