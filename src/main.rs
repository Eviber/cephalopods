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

fn compute_sum(grid: Grid, depth: usize) -> u32 {
    let mut final_sum = 0;
    let mut current = HashMap::new();
    let mut next = HashMap::new();

    current.insert(grid, 1);
    for _ in 0..depth {
        for (grid, occurrences) in &current {
            for grid in grid.possible_states() {
                if grid.dice_count() == 9 {
                    final_sum += occurrences * grid.hash();
                    continue;
                }
                next.entry(grid)
                    .and_modify(|n| *n += occurrences)
                    .or_insert(*occurrences);
            }
        }
        current.clear();
        std::mem::swap(&mut current, &mut next);
    }
    for (grid, occurrences) in current {
        final_sum += occurrences * grid.hash();
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
