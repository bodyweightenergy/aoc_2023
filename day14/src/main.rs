use std::{collections::HashMap, ops::Range, time::Instant, fmt::Display};

use tools::Opt;

const BALL: u8 = 0u8;
const CUBE: u8 = 0xffu8;
const EMPTY: u8 = 1u8;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    let lines: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|b| match b {
                    b'.' => 1u8,
                    b'O' => 0u8,
                    b'#' => 0xffu8,
                    _ => panic!("invalid char"),
                })
                .collect()
        })
        .collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut arena = Arena::new(lines);

    let start = Instant::now();
    let mut loads = vec![];
    let directions_cycle = vec![
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];
    for n in 0..250usize {
        for (i, dir) in directions_cycle.iter().enumerate() {
            // let start2 = Instant::now();
            arena.tilt(*dir);
            // let elapsed2 = Instant::now() - start2;
            // println!("Cycle {elapsed2:?}");
            let load = arena.calc_north_load();
            println!("[{}] ({dir}) Load = {}", n + i, load);
            if n > (250 - 100) {
                loads.push(load);
            }
        }
    }
    let elapsed = Instant::now() - start;
    println!("All {elapsed:?}");

    let quarters: Vec<Vec<usize>> = (0..4)
        .map(|n| loads.iter().skip(n).step_by(4).cloned().collect())
        .collect();
    for q in quarters {
        println!("{q:?}");
    }
}

struct Arena {
    data: Vec<Vec<u8>>,
    size: usize,
}

impl Arena {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        let size = data.len();
        Self { data, size }
    }

    /// Rotates all data counter-clockwise (North becomes West).
    pub fn rotate_ccw(&mut self) {
        let mut new_data = vec![vec![0u8; self.size]; self.size];
        for y in 0..self.size {
            for x in 0..self.size {
                let old_pos = (x, y);
                let new_pos = self.ccw_pos(x, y);
                new_data[new_pos.1][new_pos.0] = self.get(old_pos);
            }
        }

        self.data = new_data;
    }

    pub fn get(&self, pos: (usize, usize)) -> u8 {
        self.data[pos.1][pos.0]
    }

    pub fn set(&mut self, pos: (usize, usize), val: u8) {
        self.data[pos.1][pos.0] = val;
    }

    pub fn swap(&mut self, pos_a: (usize, usize), pos_b: (usize, usize)) {
        let val_a = self.get(pos_a);
        self.set(pos_a, self.get(pos_b));
        self.set(pos_b, val_a);
    }

    /// Gets the new coordinates of a position after one CCW rotation.
    fn ccw_pos(&self, x: usize, y: usize) -> (usize, usize) {
        let transposed = (y, x);
        let x_diff_to_max = self.size - transposed.1 - 1;
        let adjusted = (transposed.0, x_diff_to_max);
        adjusted
    }

    /// Tilts the arena west, so all balls roll that direction.
    pub fn tilt_west(&mut self) {
        for row in &mut self.data {
            let group_ranges = ranges_where(row, |v| *v == BALL || *v == EMPTY);
            for range in group_ranges {
                let slice = &mut row[range];
                slice.sort_unstable();
            }
        }
    }

    pub fn calc_north_load(&self) -> usize {
        let mut load = 0;
        for (i, row) in self.data.iter().enumerate() {
            let ball_ct = row.iter().filter(|v| **v == BALL).count();
            let coeff = self.size - i;
            load += coeff * ball_ct;
        }
        load
    }

    pub fn tilt(&mut self, direction: Direction) {
        let ccw_rotations_to_west = direction as u32;
        for _ in 0..ccw_rotations_to_west {
            self.rotate_ccw();
        }
        self.tilt_west();
        let ccw_rotations_back = 4 - ccw_rotations_to_west;
        for _ in 0..ccw_rotations_back {
            self.rotate_ccw();
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    West = 0,
    North = 1,
    East = 2,
    South = 3,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::West => 'W',
            Direction::North => 'N',
            Direction::East => 'E',
            Direction::South => 'S',
        };
        write!(f, "{c}")
    }
}

fn ranges_where<T, F>(input: &[T], predicate: F) -> Vec<Range<usize>>
where
    F: Fn(&T) -> bool,
{
    let mut ranges = vec![];
    let mut start = None;

    for (i, v) in input.iter().enumerate() {
        if predicate(v) {
            if start.is_none() {
                start = Some(i);
            }
        } else {
            if let Some(s) = start.take() {
                ranges.push(s..i);
            }
        }
    }
    if let Some(s) = start.take() {
        ranges.push(s..input.len());
    }

    ranges
}

// #[derive(Debug)]
// struct Column {
//     groups: Vec<RockGroup>,
// }

// #[derive(Debug)]
// struct RockGroup {
//     start: usize,
//     rocks: usize,
//     len: usize,
// }

// impl RockGroup {
//     pub fn load(&self, max_height: usize) -> usize {
//         let mut total_weight = 0;
//         let first_weight = max_height - self.start;

//         // gradually decrease added weight
//         for n in 0..self.rocks {
//             total_weight += first_weight - n;
//         }

//         total_weight
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn ccw() {
        let arena = Arena::new(vec![vec![0u8; 100]; 100]);

        assert_eq!(arena.ccw_pos(0, 0), (0, 99));
        assert_eq!(arena.ccw_pos(0, 99), (99, 99));
        assert_eq!(arena.ccw_pos(99, 99), (99, 0));
        assert_eq!(arena.ccw_pos(99, 0), (0, 0));

        assert_eq!(arena.ccw_pos(3, 2), (2, 96));
    }
}
