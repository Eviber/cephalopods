use itertools::Itertools;

use std::{collections::HashMap, io};

fn get_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Grid {
    grid: [[u32; 3]; 3],
}

impl Grid {
    fn from_input() -> Grid {
        let mut grid = [[0; 3]; 3];
        for row in &mut grid {
            let inputs = get_line();
            for (x, s) in inputs.split_whitespace().enumerate() {
                let value: u32 = s.parse().unwrap();
                row[x] = value;
                eprint!("{value}");
            }
            eprintln!();
        }
        Grid { grid }
    }

    fn possible_states(&self) -> Vec<Grid> {
        let mut possible_states = Vec::new();

        for y in 0..3 {
            for x in 0..3 {
                if self.grid[y][x] != 0 {
                    continue;
                }
                let mut neighbours = Vec::with_capacity(4);
                if y > 0 && self.grid[y - 1][x] != 0 && self.grid[y - 1][x] != 6 {
                    neighbours.push((x, y - 1));
                }
                if x > 0 && self.grid[y][x - 1] != 0 && self.grid[y][x - 1] != 6 {
                    neighbours.push((x - 1, y));
                }
                if y < 2 && self.grid[y + 1][x] != 0 && self.grid[y + 1][x] != 6 {
                    neighbours.push((x, y + 1));
                }
                if x < 2 && self.grid[y][x + 1] != 0 && self.grid[y][x + 1] != 6 {
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
                        sum += self.grid[y][x];
                    }
                    if sum > 6 {
                        continue;
                    }
                    valid_neighbours = true;
                    let mut new_state = *self;
                    for &(x, y) in comb {
                        new_state.grid[y][x] = 0;
                    }
                    new_state.grid[y][x] = sum;
                    possible_states.push(new_state);
                }
                if !valid_neighbours {
                    let mut new_state = *self;
                    new_state.grid[y][x] = 1;
                    possible_states.push(new_state);
                }
            }
        }
        possible_states
    }

    fn hash(&self) -> u32 {
        let mut hash = 0;

        for row in &self.grid {
            for value in row {
                hash *= 10;
                hash += value;
            }
        }
        hash
    }

    fn dice_count(&self) -> usize {
        self.grid.iter().flatten().filter(|&&d| d != 0).count()
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
type StateBuffer = [GridStateMap; 9];

fn new_state_buffer() -> StateBuffer {
    [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ]
}

fn compute_sum(grid: Grid, depth: usize) -> u32 {
    let mut final_sum = 0;
    let mut current = new_state_buffer();
    let mut next = new_state_buffer();

    if grid.hash() == 0 {
        let possible_states = grid.possible_states();
        let mut paths = [0; MAX_DEPTH + 1];
        paths[depth - 1] = 1;
        for grid in possible_states {
            let next_dice_count = 1;
            current[9 - next_dice_count].insert(grid, paths);
        }
    } else {
        let mut paths = [0; MAX_DEPTH + 1];
        paths[depth] = 1;
        current[9 - grid.dice_count()].insert(grid, paths);
    }
    loop {
        let mut was_empty = true;
        for i in 0..9 {
            let grid_states = std::mem::take(&mut current[i]);
            if grid_states.is_empty() {
                continue;
            }
            was_empty = false;
            for (grid, path) in grid_states {
                let possible_states = grid.possible_states();
                if possible_states.is_empty() {
                    final_sum += path.into_iter().sum::<u32>() * grid.hash();
                    continue;
                }
                final_sum += path[0] * grid.hash();
                let current_dice_count = grid.dice_count();
                assert_eq!(current_dice_count, 9 - i);
                for grid in possible_states {
                    let next_dice_count = grid.dice_count();
                    let p = if next_dice_count > current_dice_count {
                        next[9 - next_dice_count].entry(grid).or_insert_with(|| [0; MAX_DEPTH + 1])
                    } else {
                        assert!(next_dice_count < current_dice_count);
                        assert!(next_dice_count < 9 - i);
                        current[9 - next_dice_count].entry(grid).or_insert_with(|| [0; MAX_DEPTH + 1])
                    };
                    for (i, n) in p.iter_mut().enumerate().take(MAX_DEPTH) {
                        *n += path[i + 1];
                    }
                }
            }
        }
        if was_empty {
            break;
        }
        std::mem::swap(&mut current, &mut next);
        // next.iter_mut().for_each(|gs| gs.clear());
    }
    final_sum % (1 << 30)
}

fn main() {
    let depth: usize = get_line().trim().parse().unwrap();
    let grid = Grid::from_input();

    let final_sum = compute_sum(grid, depth);
    println!("{final_sum}");
}
