mod bitboard;

use itertools::Itertools;

use std::io;

fn get_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

#[derive(Clone, Copy, Debug)]
struct State {
    grid: [[u32; 3]; 3]
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

    fn possible_states(&self) -> Vec<State> {
        let mut possible_states = Vec::new();

        for y in 0..3 {
            for x in 0..3 {
                if self.grid[y][x] != 0 {
                    continue;
                }
                let mut neighbours = Vec::with_capacity(4);
                if y > 0 && self.grid[y-1][x] != 0 && self.grid[y-1][x] != 6 {
                    neighbours.push((x, y-1));
                }
                if x > 0 && self.grid[y][x-1] != 0 && self.grid[y][x-1] != 6 {
                    neighbours.push((x-1, y));
                }
                if y < 2 && self.grid[y+1][x] != 0 && self.grid[y+1][x] != 6 {
                    neighbours.push((x, y+1));
                }
                if x < 2 && self.grid[y][x+1] != 0 && self.grid[y][x+1] != 6 {
                    neighbours.push((x+1, y));
                }
                let mut valid_neighbours = false;
                for comb in neighbours.iter().combinations(4) {
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
                    new_state.grid[y][x] = 1;
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
}

fn compute_sum(grid: State, depth: u32, sum: &mut u32) {
    if depth == 0 {
        *sum = (*sum + grid.hash()) % (1<<30);
        return;
    }
    let possible_states = grid.possible_states();
    if possible_states.is_empty() {
        *sum = (*sum + grid.hash()) % (1<<30);
        return;
    }
    for state in possible_states {
        compute_sum(state, depth - 1, sum);
    }
}

fn main() {
    let depth: u32 = get_line().trim().parse().unwrap();
    let grid = State::from_input();

    let mut final_sum = 0;
    compute_sum(grid, depth, &mut final_sum);
    println!("{final_sum}");
}
