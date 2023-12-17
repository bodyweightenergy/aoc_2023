use std::{collections::HashMap, fmt::Debug, fmt::Display};

use tools::Opt;

const SLASH: u8 = 0x7F;
const FSLASH: u8 = 0x7E;
const V: u8 = 0x7D;
const H: u8 = 0x7C;
const THRESH: u8 = 0x70;

fn main() {
    let opt = Opt::load();
    let input = opt.input().lines();
    let lines = opt.lines();

    let mut arena = Arena::new(lines);

    let dir = arena.first_tile();
    let start_pos = Position::new(0, 0);

    let mut starts: Vec<(Position, Direction)> = vec![(start_pos, dir)];
    loop {
    // for _ in 0..10 {
        let mut new_starts = vec![];
        for (start_pos, dir) in &starts {
            let tiles = arena.get_tiles_in_direction(&start_pos, *dir);
            println!("{start_pos} + {dir} = {tiles:?}");
            // Abort if reached edge
            if tiles.len() == 0 {
                continue;
            }
            if let Some(last_tile) = arena.run_through(&tiles) {
                let last_mirror = MirrorShape::from_byte(arena.get_tile(&last_tile)).unwrap();
                println!("Last = {last_mirror:?}");

                for new_dir in last_mirror.redirections(&dir.reverse()) {
                    println!("+ {last_tile} -> {new_dir}");
                    new_starts.push((last_tile.clone(), new_dir));
                }
            }
            else {
                println!("Last = Edge");
            }
            // arena.print();
            // println!("------------------------------------------------------------");

        }

        starts = new_starts;

        if starts.len() == 0 {
            break;
        }
    }

    arena.print();

    println!("Total energized tiles = {}", arena.total_energized());
}

struct Arena {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Arena {
    pub fn new(lines: Vec<String>) -> Self {
        let data = lines
            .iter()
            .map(|line| line.chars().map(|c| Self::convert_char(c)).collect())
            .collect();

        let max_width = lines[0].len();
        let max_height = lines.len();

        Arena {
            data,
            width: max_width,
            height: max_height,
        }
    }

    fn convert_char(c: char) -> u8 {
        match c {
            '/' => FSLASH,
            '\\' => SLASH,
            '|' => V,
            '-' => H,
            _ => 0u8,
        }
    }

    pub fn first_tile(&mut self) -> Direction {
        let first_pos = Position::new(0, 0);

        if self.space(&first_pos).is_some() {
            self.set_tile(&first_pos, 1);
            Direction::E
        } else {
            let first_tile = self.get_tile(&first_pos);
            match first_tile {
                SLASH => Direction::S,
                FSLASH => Direction::N,
                H => Direction::S,
                V => Direction::E,
                _ => Direction::E,
            }
        }
    }

    pub fn get_tile(&self, pos: &Position) -> u8 {
        self.data[pos.y][pos.x]
    }

    pub fn set_tile(&mut self, pos: &Position, val: u8) {
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
                    if self.space(&pos).is_none() {
                        break;
                    }
                }
            }
            Direction::S => {
                for y in start.y + 1..self.height {
                    let pos = Position::new(start.x, y);
                    positions.push(pos.clone());
                    if self.space(&pos).is_none() {
                        break;
                    }
                }
            }
            Direction::W => {
                for x in (0..start.x).rev() {
                    let pos = Position::new(x, start.y);
                    positions.push(pos.clone());
                    if self.space(&pos).is_none() {
                        break;
                    }
                }
            }
            Direction::E => {
                for x in start.x + 1..self.width {
                    let pos = Position::new(x, start.y);
                    positions.push(pos.clone());
                    if self.space(&pos).is_none() {
                        break;
                    }
                }
            }
        }

        positions
    }

    pub fn space(&self, pos: &Position) -> Option<u8> {
        let val = self.get_tile(pos);
        if val < THRESH {
            Some(val)
        } else {
            None
        }
    }

    pub fn run_through(&mut self, tiles: &[Position]) -> Option<Position> {
        for (i, pos) in tiles.iter().enumerate() {
            if let Some(v) = self.space(pos) {
                // This indicates an infinite loop
                if v == 9 {
                    return None;
                }
                self.set_tile(pos, 1 + 1);
            }
        }

        let last_pos = tiles.last().unwrap();
        if self.space(last_pos).is_none() {
            let last_val = self.get_tile(last_pos);
            // A mirror that is energized is its value OR'd with 0x80
            self.set_tile(last_pos, last_val | 0x80);
            Some(last_pos.clone())
        } else {
            None
        }
    }

    pub fn total_energized(&self) -> usize {
        let mut acc = 0usize;
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x, y);
                if let Some(energy) = self.space(&pos) {
                    if energy > 0 {
                        acc += 1;
                    }
                }
                else {
                    // Check if mirror is energized
                    let val = self.get_tile(&pos);
                    if val & 0x80 > 0 {
                        acc += 1;
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
                let v = self.get_tile(&Position::new(x, y));
                let c = match v {
                    SLASH => '\\',
                    FSLASH => '/',
                    H => '-',
                    V => '|',
                    _ => v
                        .to_string()
                        .chars()
                        .collect::<Vec<char>>()
                        .last()
                        .unwrap()
                        .clone(),
                };
                s.push(c);
            }
            s.push_str("\n");
        }
        print!("{}", s);
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
    pub fn to_byte(&self) -> u8 {
        match self {
            MirrorShape::Slash => SLASH,
            MirrorShape::ForwardSlash => FSLASH,
            MirrorShape::SplitV => V,
            MirrorShape::SplitH => H,
        }
    }

    pub fn from_byte(input: u8) -> Option<MirrorShape> {
        let unenergized = input & 0x7F;
        match unenergized {
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
