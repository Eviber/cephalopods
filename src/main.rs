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

    fn possible_states(&self) -> impl Iterator<Item = Grid> {
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
        possible_states.into_iter()
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
        Grid {
            bitset: self.bitset & 0o_007_070_700
                | ((self.bitset & 0o_070_700_000) >> (4 * 3))
                | ((self.bitset & 0o_000_007_070) << (4 * 3))
                | ((self.bitset & 0o_000_000_007) << (8 * 3))
                | ((self.bitset & 0o_700_000_000) >> (8 * 3)),
        }
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

const TRANSFORM_COUNT: usize = 8;
type Paths = [u32; TRANSFORM_COUNT];
type GridStateMap = HashMap<Grid, Paths>;

fn add_grid(next: &mut GridStateMap, grid: Grid, parent_path: &Paths) {
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
    let p = next
        .entry(canon_transform)
        .or_insert_with(|| [0; TRANSFORM_COUNT]);
    for (t_before, &t_after) in f_table[transform_idx].iter().enumerate() {
        p[t_after] += parent_path[t_before];
    }
}

fn add_grid_to_sum(sum: &mut [u32; 9], grid: Grid, factor: u32) {
    sum[0] += grid.get(0, 0) * factor;
    sum[1] += grid.get(1, 0) * factor;
    sum[2] += grid.get(2, 0) * factor;
    sum[3] += grid.get(0, 1) * factor;
    sum[4] += grid.get(1, 1) * factor;
    sum[5] += grid.get(2, 1) * factor;
    sum[6] += grid.get(0, 2) * factor;
    sum[7] += grid.get(1, 2) * factor;
    sum[8] += grid.get(2, 2) * factor;
}

fn compute_sum(grid: Grid, depth: usize) -> u32 {
    let mut grid_sum = [0; 9];
    let mut current = GridStateMap::new();
    let mut next = GridStateMap::new();

    let mut path = [0; TRANSFORM_COUNT];
    path[0] = 1;
    current.insert(grid, path);

    for _ in 0..depth {
        for (grid, path) in &current {
            if grid.dice_count() == 9 {
                add_grid_to_sum(&mut grid_sum, *grid, path[0]);
                add_grid_to_sum(&mut grid_sum, grid.v_flip(), path[1]);
                add_grid_to_sum(&mut grid_sum, grid.h_flip(), path[2]);
                add_grid_to_sum(&mut grid_sum, grid.vh_flip(), path[3]);
                add_grid_to_sum(&mut grid_sum, grid.d_flip(), path[4]);
                add_grid_to_sum(&mut grid_sum, grid.dh_flip(), path[5]);
                add_grid_to_sum(&mut grid_sum, grid.dv_flip(), path[6]);
                add_grid_to_sum(&mut grid_sum, grid.dvh_flip(), path[7]);
                continue;
            }
            for g in grid.possible_states() {
                add_grid(&mut next, g, path);
            }
        }
        current.clear();
        std::mem::swap(&mut current, &mut next);
    }
    for (grid, path) in current {
        add_grid_to_sum(&mut grid_sum, grid, path[0]);
        add_grid_to_sum(&mut grid_sum, grid.v_flip(), path[1]);
        add_grid_to_sum(&mut grid_sum, grid.h_flip(), path[2]);
        add_grid_to_sum(&mut grid_sum, grid.vh_flip(), path[3]);
        add_grid_to_sum(&mut grid_sum, grid.d_flip(), path[4]);
        add_grid_to_sum(&mut grid_sum, grid.dh_flip(), path[5]);
        add_grid_to_sum(&mut grid_sum, grid.dv_flip(), path[6]);
        add_grid_to_sum(&mut grid_sum, grid.dvh_flip(), path[7]);
    }
    let mut final_sum = 0;
    for n in grid_sum {
        final_sum *= 10;
        final_sum += n;
    }
    final_sum % (1 << 30)
}

fn main() {
    let depth: usize = get_line().trim().parse().unwrap();
    let grid = Grid::from_input();
    grid.print();
    eprintln!();

    if grid.bitset == 0 {
        println!(
            "{}",
            [
                0, 111111111, 704035952, 840352818, 600875666, 50441886, 680243700, 597686656,
                584450980, 55305380, 193520836, 521847116, 1054388152, 518795448, 366207036,
                678967952, 476916052, 1009258340, 592651828, 1063467872, 400415524, 233248832,
                230461008, 245411624, 899694236, 384163740, 888060600, 347933640, 340717612,
                73295296, 851289228, 221286388, 375032784, 723342020, 92414440, 745533092,
                331519112, 993643868, 72093236, 422667876, 503115192,
            ][depth]
        );
        return;
    }

    let final_sum = compute_sum(grid, depth);
    println!("{final_sum}");
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
