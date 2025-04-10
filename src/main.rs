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
                eprint!("{value}");
            }
            eprintln!();
        }
        grid
    }

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

    fn hash(&self) -> u32 {
        let mut hash = 0;

        for n in 0..9 {
            hash = hash * 10 + self.get(n % 3, n / 3);
        }
        hash
    }

    fn dice_count(&self) -> usize {
        self.occupied().count_ones() as usize
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
type PathCountsByDepth = [u32; MAX_DEPTH + 1];
type GridStateMap = HashMap<Grid, PathCountsByDepth>;

struct StateBuffer([Option<GridStateMap>; 9]);

impl StateBuffer {
    fn new() -> Self {
        StateBuffer([
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
            Some(HashMap::new()),
        ])
    }

    #[inline]
    fn insert(&mut self, idx: usize, grid: Grid, paths: PathCountsByDepth) {
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
    fn entry(&mut self, idx: usize, grid: Grid) -> &mut PathCountsByDepth {
        self.0[idx]
            .as_mut()
            .unwrap()
            .entry(grid)
            .or_insert_with(|| [0; MAX_DEPTH + 1])
    }

    #[inline]
    fn put(&mut self, idx: usize, grid_states: GridStateMap) {
        self.0[idx] = Some(grid_states);
    }
}

fn compute_sum(grid: Grid, depth: usize) -> u32 {
    let mut final_sum = 0;
    let mut state_buffer = StateBuffer::new();

    let mut paths = [0; MAX_DEPTH + 1];
    paths[depth] = 1;
    let idx = if grid.hash() == 0 {
        7 // we can put an initial empty grid anywhere except at index 0 or 8
    } else {
        9 - grid.dice_count()
    };
    state_buffer.insert(idx, grid, paths);
    loop {
        let mut was_empty = true;
        for i in 0..9 {
            if state_buffer.is_empty(i) {
                continue;
            }
            let mut grid_states = state_buffer.take(i);
            was_empty = false;
            for (grid, path) in &grid_states {
                if i == 0 {
                    final_sum += path.iter().sum::<u32>() * grid.hash();
                    continue;
                }
                final_sum += path[0] * grid.hash();
                if path.iter().skip(1).all(|&n| n == 0) {
                    continue;
                }
                for g in grid.possible_states() {
                    let next_dice_count = g.dice_count();
                    let p = {
                        if next_dice_count == 9 - i {
                            eprintln!("{:09}", grid.hash());
                            eprintln!("{:09}", g.hash());
                            panic!()
                        }
                        state_buffer.entry(9 - next_dice_count, g)
                    };
                    for (i, n) in p.iter_mut().enumerate().take(MAX_DEPTH) {
                        *n += path[i + 1];
                    }
                }
            }
            grid_states.clear();
            state_buffer.put(i, grid_states);
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

    let final_sum = compute_sum(grid, depth);
    println!("{final_sum}");
}
