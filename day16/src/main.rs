use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fmt::Display,
};

use itertools::Itertools;
use tools::Opt;

const SLASH: char = '\\';
const FSLASH: char = '/';
const V: char = '|';
const H: char = '-';

fn main() {
    let opt = Opt::load();
    let input = opt.input().lines();
    let lines = opt.lines();
    let arena = Arena::new(&lines);

    let mut starting_points = vec![];
    let mut arena_energy = vec![];

    for x in 0..arena.width {
        for y in 0..arena.height {
            if x == 0 || x == arena.width - 1 || y == 0 || y == arena.height - 1 {
                starting_points.push(Position::new(x, y));
            }
        }
    }

    for pt in &starting_points {
        arena_energy.push((pt.clone(), calc_arena(&lines, pt.clone())));
    }

    for (s, e) in &arena_energy {
        println!("{s} => {e}");
    }

    println!("Calculated for {} starting points.", &starting_points.len());

    println!(
        "Max energy = {}",
        arena_energy.iter().map(|o| o.1).max().unwrap()
    );
}

/// Runs scenario with starting tile location, and returns number of energized tiles.
fn calc_arena(lines: &[String], starting_pos: Position) -> usize {
    let ref_arena = Arena::new(&lines);

    // Possible directions for this starting point
    let start_dirs = ref_arena.first_directions(&starting_pos);

    let mut total_energy: Vec<usize> = vec![];

    for start_dir in start_dirs {
        let mut arena = Arena::new(&lines);
        let first_dirs = arena.first_tile(&starting_pos, start_dir);
        let mut starts: Vec<(Position, Direction)> = first_dirs
            .iter()
            .map(|d| (starting_pos.clone(), *d))
            .collect();

        loop {
            // for _ in 0..10 {
            let mut new_starts = vec![];
            for (start_pos, dir) in &starts {
                let tiles = arena.get_tiles_in_direction(&start_pos, *dir);
                if let Some(last_tile) = arena.run_through(dir, &tiles) {
                    // let last_mirror = MirrorShape::from_char(arena.get(&last_tile)).unwrap();
                    if let Tile::Object { shape, .. } = arena.get(&last_tile) {
                        for new_dir in shape.redirections(&dir.reverse()) {
                            new_starts.push((last_tile.clone(), new_dir));
                        }
                    }
                }
            }

            starts = new_starts;

            if starts.len() == 0 {
                break;
            }
        }

        let total_energized = arena.total_energized();

        total_energy.push(total_energized)
    }
    total_energy.iter().max().unwrap().clone()
}

struct Arena {
    data: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Arena {
    pub fn new(lines: &[String]) -> Self {
        let data = lines
            .iter()
            .map(|line| line.chars().map(|c| Tile::new(c)).collect())
            .collect();

        let max_width = lines[0].len();
        let max_height = lines.len();

        Arena {
            data,
            width: max_width,
            height: max_height,
        }
    }

    pub fn first_directions(&self, first_pos: &Position) -> Vec<Direction> {
        let mut first_directions = vec![];
        // West Edge
        if first_pos.x == 0 {
            first_directions.push(Direction::E);
        }
        // East Edge
        if first_pos.x == self.width - 1 {
            first_directions.push(Direction::W);
        }
        // North Edge
        if first_pos.y == 0 {
            first_directions.push(Direction::S);
        }
        // South Edge
        if first_pos.y == self.height - 1 {
            first_directions.push(Direction::N);
        }
        first_directions
    }

    pub fn first_tile(&mut self, first_pos: &Position, first_dir: Direction) -> Vec<Direction> {
        match self.get_mut(&first_pos) {
            Tile::Object { shape, energized } => {
                *energized = true;
                shape.redirections(&first_dir)
            }
            Tile::Space(set) => {
                set.push(first_dir);
                vec![first_dir]
            }
        }
    }

    pub fn get(&self, pos: &Position) -> &Tile {
        &self.data[pos.y][pos.x]
    }

    pub fn get_mut(&mut self, pos: &Position) -> &mut Tile {
        &mut self.data[pos.y][pos.x]
    }

    pub fn set_tile(&mut self, pos: &Position, val: Tile) {
        self.data[pos.y][pos.x] = val;
    }

    /// Gets all tile positions from a start point in a single direction.
    /// The returned positions do not include the start point, and do include the end point.
    /// The end point can either be a mirror/splitter, or an edge empty space.
    pub fn get_tiles_in_direction(&self, start: &Position, direction: Direction) -> Vec<Position> {
        let mut positions = vec![];

        match direction {
            Direction::N => {
                // Walking north
                for y in (0..start.y).rev() {
                    let pos = Position::new(start.x, y);
                    positions.push(pos.clone());
                    if let Tile::Object { .. } = self.get(&pos) {
                        break;
                    }
                }
            }
            Direction::S => {
                for y in start.y + 1..self.height {
                    let pos = Position::new(start.x, y);
                    positions.push(pos.clone());
                    if let Tile::Object { .. } = self.get(&pos) {
                        break;
                    }
                }
            }
            Direction::W => {
                for x in (0..start.x).rev() {
                    let pos = Position::new(x, start.y);
                    positions.push(pos.clone());
                    if let Tile::Object { .. } = self.get(&pos) {
                        break;
                    }
                }
            }
            Direction::E => {
                for x in start.x + 1..self.width {
                    let pos = Position::new(x, start.y);
                    positions.push(pos.clone());
                    if let Tile::Object { .. } = self.get(&pos) {
                        break;
                    }
                }
            }
        }

        positions
    }

    // pub fn space(&self, pos: &Position) -> Option<u8> {
    //     let val = self.get(pos);
    //     if val < THRESH {
    //         Some(val)
    //     } else {
    //         None
    //     }
    // }

    pub fn run_through(&mut self, dir: &Direction, tiles: &[Position]) -> Option<Position> {
        for pos in tiles {
            match self.get_mut(&pos) {
                Tile::Space(set) => {
                    if set.contains(dir) {
                        // Beam already passed through this direction, cancel this branch
                        return None;
                    } else {
                        if !set.contains(dir) {
                            set.push(*dir);
                        };
                    }
                }
                Tile::Object { energized, .. } => {
                    *energized = true;
                    return Some(pos.clone());
                }
            }
        }
        None
    }

    pub fn total_energized(&self) -> usize {
        let mut acc = 0usize;
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x, y);
                match self.get(&pos) {
                    Tile::Object { energized, .. } => {
                        if *energized {
                            acc += 1
                        }
                    }
                    Tile::Space(set) => {
                        if set.len() > 0 {
                            acc += 1
                        }
                    }
                }
            }
        }
        acc
    }

    pub fn print(&self) {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.get(&Position::new(x, y));
                let c = match v {
                    Tile::Object { shape, .. } => shape.to_char().to_string(),
                    Tile::Space(set) => {
                        if set.len() == 0 {
                            ".".to_owned()
                        } else if set.len() == 1 {
                            match set[0] {
                                Direction::N => "^",
                                Direction::S => "v",
                                Direction::W => "<",
                                Direction::E => ">",
                            }
                            .to_owned()
                        } else {
                            set.len().to_string()
                        }
                    }
                };
                s.push_str(&c);
            }
            s.push_str("\n");
        }
        print!("{}", s);
    }

    pub fn print_energized(&self) {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.get(&Position::new(x, y));
                let c = if match v {
                    Tile::Object { energized, .. } => *energized,
                    Tile::Space(set) => !set.is_empty(),
                } {
                    "#"
                } else {
                    "."
                };
                s.push_str(&c);
            }
            s.push_str("\n");
        }
        print!("{}", s);
    }
}

enum Tile {
    Object { shape: MirrorShape, energized: bool },
    Space(Vec<Direction>),
}

impl Tile {
    pub fn new(c: char) -> Self {
        if c == '.' {
            Self::Space(Vec::new())
        } else {
            let shape = MirrorShape::from_char(c).unwrap();
            Self::Object {
                shape,
                energized: false,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mirror {
    /// Tile pipe shape
    pub shape: MirrorShape,
    pub position: Position,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    /// Gets vector of all directions.
    pub fn all() -> Vec<Direction> {
        vec![Direction::N, Direction::S, Direction::W, Direction::E]
    }

    /// Gets the reverse direction.
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::N => 'N',
            Direction::S => 'S',
            Direction::W => 'W',
            Direction::E => 'E',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MirrorShape {
    Slash,
    ForwardSlash,
    SplitV,
    SplitH,
}

impl MirrorShape {
    pub fn to_char(&self) -> char {
        match self {
            MirrorShape::Slash => SLASH,
            MirrorShape::ForwardSlash => FSLASH,
            MirrorShape::SplitV => V,
            MirrorShape::SplitH => H,
        }
    }

    pub fn from_char(input: char) -> Option<MirrorShape> {
        match input {
            SLASH => Some(MirrorShape::Slash),
            FSLASH => Some(MirrorShape::ForwardSlash),
            V => Some(MirrorShape::SplitV),
            H => Some(MirrorShape::SplitH),
            _ => None,
        }
    }

    /// Gets the directions this mirror/splitter will redirect the beam.
    /// `incoming` is the direction relative to the object
    /// (e.g. `Direction::North` means southward beam coming in from the north of the object).
    pub fn redirections(&self, incoming: &Direction) -> Vec<Direction> {
        use Direction::{E, N, S, W};
        use MirrorShape::{ForwardSlash, Slash, SplitH, SplitV};
        match self {
            Slash => match incoming {
                N => vec![E],
                S => vec![W],
                W => vec![S],
                E => vec![N],
            },
            ForwardSlash => match incoming {
                N => vec![W],
                S => vec![E],
                W => vec![N],
                E => vec![S],
            },
            SplitH => match incoming {
                N => vec![W, E],
                S => vec![W, E],
                W => vec![E],
                E => vec![W],
            },
            SplitV => match incoming {
                N => vec![S],
                S => vec![N],
                W => vec![N, S],
                E => vec![N, S],
            },
        }
    }
}

// impl Display for MirrorShape {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.to_byte())
//     }
// }

// impl Debug for MirrorShape {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Display::fmt(self, f)
//     }
// }
