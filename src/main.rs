use std::{collections::HashMap, io};

fn get_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

#[derive(Clone, Copy, Debug)]
struct State {
    grid: [[u32; 3]; 3],
}

impl State {
    fn from_input() -> State {
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
        State { grid }
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
}

fn compute_sum(
    grid: State,
    depth: u32,
    total_sum: &mut u32,
    solved: &mut HashMap<(u32, u32), u32>,
) {
    let current_hash = grid.hash();
    if depth == 0 {
        *total_sum = (*total_sum + current_hash) % (1 << 30);
        return;
    }
    if let Some(sum) = solved.get(&(current_hash, depth)) {
        *total_sum = (*total_sum + sum) % (1 << 30);
        return;
    }
    let mut grid_copy = grid;
    let mut sum = 0;
    for y in 0..3 {
        for x in 0..3 {
            if grid.grid[y][x] != 0 {
                continue;
            }
            let mut inner_sum = 0;
            for &comb in get_combs_for_cell(x, y) {
                for &(cx, cy) in comb {
                    if grid.grid[cy][cx] == 0 {
                        break;
                    }
                    grid_copy.grid[y][x] += grid.grid[cy][cx];
                    grid_copy.grid[cy][cx] = 0;
                }
                if grid_copy.grid[y][x] > 6 {
                    continue;
                }
                compute_sum(grid_copy, depth - 1, &mut inner_sum, solved);
                grid_copy = grid;
            }
            if inner_sum == 0 {
                grid_copy.grid[y][x] = 1;
                compute_sum(grid_copy, depth - 1, &mut inner_sum, solved);
            }
            sum += inner_sum;
        }
    }
    if sum == 0 {
        *total_sum = (*total_sum + current_hash) % (1 << 30);
        return;
    }
    solved.insert((current_hash, depth), sum);
    *total_sum = (*total_sum + sum) % (1 << 30);
}

// Define a type alias for clarity
type Coord = (usize, usize);
type Combination = &'static [Coord];
type CellCombinations = &'static [Combination];

// Position (0,0) - top left corner - has 2 neighbors: (1,0) and (0,1)
const COMBS_0_0: [&[Coord]; 3] = [
    &[(1, 0), (0, 1)],     // Both neighbors
    &[(1, 0)],             // Right neighbor only
    &[(0, 1)],             // Bottom neighbor only
];

// Position (1,0) - top middle - has 3 neighbors: (0,0), (2,0), and (1,1)
const COMBS_1_0: [&[Coord]; 7] = [
    &[(0, 0), (2, 0), (1, 1)],     // All 3 neighbors
    &[(0, 0), (2, 0)],             // Left and right
    &[(0, 0), (1, 1)],             // Left and bottom
    &[(2, 0), (1, 1)],             // Right and bottom
    &[(0, 0)],                     // Left only
    &[(2, 0)],                     // Right only
    &[(1, 1)],                     // Bottom only
];

// Position (2,0) - top right corner - has 2 neighbors: (1,0) and (2,1)
const COMBS_2_0: [&[Coord]; 3] = [
    &[(1, 0), (2, 1)],     // Both neighbors
    &[(1, 0)],             // Left neighbor only
    &[(2, 1)],             // Bottom neighbor only
];

// Position (0,1) - middle left - has 3 neighbors: (0,0), (1,1), and (0,2)
const COMBS_0_1: [&[Coord]; 7] = [
    &[(0, 0), (1, 1), (0, 2)],     // All 3 neighbors
    &[(0, 0), (1, 1)],             // Top and right
    &[(0, 0), (0, 2)],             // Top and bottom
    &[(1, 1), (0, 2)],             // Right and bottom
    &[(0, 0)],                     // Top only
    &[(1, 1)],                     // Right only
    &[(0, 2)],                     // Bottom only
];

// Position (1,1) - center - has 4 neighbors: (0,1), (1,0), (2,1), and (1,2)
const COMBS_1_1: [&[Coord]; 11] = [
    &[(0, 1), (1, 0), (2, 1), (1, 2)],     // All 4 neighbors
    &[(0, 1), (1, 0), (2, 1)],             // Left, top, right
    &[(0, 1), (1, 0), (1, 2)],             // Left, top, bottom
    &[(0, 1), (2, 1), (1, 2)],             // Left, right, bottom
    &[(1, 0), (2, 1), (1, 2)],             // Top, right, bottom
    &[(0, 1), (1, 0)],                     // Left, top
    &[(0, 1), (2, 1)],                     // Left, right
    &[(0, 1), (1, 2)],                     // Left, bottom
    &[(1, 0), (2, 1)],                     // Top, right
    &[(1, 0), (1, 2)],                     // Top, bottom
    &[(2, 1), (1, 2)],                     // Right, bottom
];

// Position (2,1) - middle right - has 3 neighbors: (2,0), (1,1), and (2,2)
const COMBS_2_1: [&[Coord]; 7] = [
    &[(2, 0), (1, 1), (2, 2)],     // All 3 neighbors
    &[(2, 0), (1, 1)],             // Top and left
    &[(2, 0), (2, 2)],             // Top and bottom
    &[(1, 1), (2, 2)],             // Left and bottom
    &[(2, 0)],                     // Top only
    &[(1, 1)],                     // Left only
    &[(2, 2)],                     // Bottom only
];

// Position (0,2) - bottom left corner - has 2 neighbors: (0,1) and (1,2)
const COMBS_0_2: [&[Coord]; 3] = [
    &[(0, 1), (1, 2)],     // Both neighbors
    &[(0, 1)],             // Top neighbor only
    &[(1, 2)],             // Right neighbor only
];

// Position (1,2) - bottom middle - has 3 neighbors: (0,2), (1,1), and (2,2)
const COMBS_1_2: [&[Coord]; 7] = [
    &[(0, 2), (1, 1), (2, 2)],     // All 3 neighbors
    &[(0, 2), (1, 1)],             // Left and top
    &[(0, 2), (2, 2)],             // Left and right
    &[(1, 1), (2, 2)],             // Top and right
    &[(0, 2)],                     // Left only
    &[(1, 1)],                     // Top only
    &[(2, 2)],                     // Right only
];

// Position (2,2) - bottom right corner - has 2 neighbors: (1,2) and (2,1)
const COMBS_2_2: [&[Coord]; 3] = [
    &[(1, 2), (2, 1)],     // Both neighbors
    &[(1, 2)],             // Left neighbor only
    &[(2, 1)],             // Top neighbor only
];

// Global lookup array for all positions
const ALL_COMBS: [CellCombinations; 9] = [
    &COMBS_0_0, &COMBS_1_0, &COMBS_2_0,
    &COMBS_0_1, &COMBS_1_1, &COMBS_2_1,
    &COMBS_0_2, &COMBS_1_2, &COMBS_2_2,
];

// Helper function to get combinations for a specific cell
#[inline]
fn get_combs_for_cell(x: usize, y: usize) -> CellCombinations {
    ALL_COMBS[y * 3 + x]
}

fn main() {
    let depth: u32 = get_line().trim().parse().unwrap();
    let grid = State::from_input();

    let mut solved = HashMap::new();
    let mut final_sum = 0;
    compute_sum(grid, depth, &mut final_sum, &mut solved);
    println!("{final_sum}");
}
