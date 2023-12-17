use std::{collections::HashMap, fmt::Debug, fmt::Display};

use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input().lines();
    // let lines = opt.lines();
}

pub struct Arena {
    pub pipe_map: HashMap<Position, Mirror>,
    pub max_width: usize,
    pub max_height: usize,
}

impl Arena {
    pub fn new(lines: Vec<&str>) -> Self {
        let mut pipe_map = HashMap::new();
        let mut start_pos = None;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Position { x, y };
                // Start tile
                if c == 'S' {
                    start_pos = Some(position.clone());
                } else if let Some(pipe_type) = MirrorShape::from_char(c) {
                    let pipe = Mirror {
                        pipe_type,
                        position: position.clone(),
                    };
                    pipe_map.insert(position, pipe);
                }
            }
        }

        let max_width = lines[0].len();
        let max_height = lines.len();

        println!("Start Position = {:?}", start_pos);

        Arena {
            pipe_map,
            max_width,
            max_height,
        }
    }

    // /// Gets a neighbor tile, if it exists.
    // pub fn get_adjacent(&self, position: &Position, direction: Direction) -> Option<Position> {
    //     let tile_pos = &position;
    //     let x = tile_pos.x;
    //     let y = tile_pos.y;
    //     let adj_pos = match direction {
    //         Direction::N => {
    //             if y == 0 {
    //                 return None;
    //             } else {
    //                 Position { x, y: y - 1 }
    //             }
    //         }
    //         Direction::S => {
    //             if y == self.max_height {
    //                 return None;
    //             } else {
    //                 Position { x, y: y + 1 }
    //             }
    //         }
    //         Direction::W => {
    //             if x == 0 {
    //                 return None;
    //             } else {
    //                 Position {
    //                     x: tile_pos.x - 1,
    //                     y,
    //                 }
    //             }
    //         }
    //         Direction::E => {
    //             if x == self.max_width {
    //                 return None;
    //             } else {
    //                 Position {
    //                     x: tile_pos.x + 1,
    //                     y,
    //                 }
    //             }
    //         }
    //     };
    //     Some(adj_pos)
    // }

    // pub fn get_adjacents(&self, position: &Position) -> HashMap<Direction, Position> {
    //     let mut adjacents = HashMap::new();
    //     for dir in Direction::all() {
    //         if let Some(neighbor) = self.get_adjacent(position, dir) {
    //             adjacents.insert(dir, neighbor);
    //         }
    //     }

    //     adjacents
    // }

    // /// Gets all neighbors of the selected tile.
    // pub fn get_neighbors(&self, tile: &Mirror) -> HashMap<Direction, &Mirror> {
    //     let mut adjacents = HashMap::new();
    //     for dir in Direction::all() {
    //         if let Some(neighbor) = self.get_adjacent(&tile.position, dir) {
    //             adjacents.insert(dir, neighbor);
    //         }
    //     }
    //     // let valid_dirs = tile.pipe_type.redirections();
    //     // for dir in Direction::all() {
    //     //     if !valid_dirs.contains(&dir) {
    //     //         adjacents.remove(&dir);
    //     //     }
    //     // }

    //     let mut neighbors = HashMap::new();
    //     for (dir, pos) in adjacents {
    //         if let Some(neighbor) = self.pipe_map.get(&pos) {
    //             neighbors.insert(dir, neighbor);
    //         }
    //     }
    //     neighbors
    // }

    /// Gets the next 
    pub fn get_next_mirror(&self, source_pos: Position, beam_direction: Direction) -> Option<Position> {
        todo!()
    }

    pub fn get_tile(&self, pos: &Position) -> Option<&Mirror> {
        self.pipe_map.get(&pos)
    }

    pub fn print(&self) {
        let mut output = String::new();
        for y in 0..self.max_height {
            for x in 0..self.max_width {
                let pos = Position { x, y };
                if let Some(tile) = self.pipe_map.get(&pos) {
                    output.push(tile.pipe_type.to_char());
                } else {
                    output.push('.');
                }
            }
            output.push_str("\r\n");
        }

        println!("{}", output);
    }

    pub fn print_selection<F>(&self, predicate: F)
    where
        F: Fn(&Position) -> bool,
    {
        let mut output = String::new();
        for y in 0..self.max_height {
            for x in 0..self.max_width {
                let pos = Position { x, y };
                if predicate(&pos) {
                    if let Some(tile) = self.pipe_map.get(&pos) {
                        output.push(tile.pipe_type.to_char());
                    } else {
                        output.push('.');
                    }
                } else {
                    output.push(' ');
                }
            }
            output.push_str("\r\n");
        }

        println!("{}", output);
    }
}

#[derive(Debug, Clone)]
pub struct Mirror {
    /// Tile pipe shape
    pub pipe_type: MirrorShape,
    pub position: Position,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MirrorShape {
    Slash,
    ForwardSlash,
    SplitV,
    SplitH,
}

impl MirrorShape {
    pub fn to_char(&self) -> char {
        match self {
            MirrorShape::Slash => '\\',
            MirrorShape::ForwardSlash => '/',
            MirrorShape::SplitV => '|',
            MirrorShape::SplitH => '-',
        }
    }

    pub fn from_char(input: char) -> Option<MirrorShape> {
        match input {
            '\\' => Some(MirrorShape::Slash),
            '/' => Some(MirrorShape::ForwardSlash),
            '|' => Some(MirrorShape::SplitV),
            '-' => Some(MirrorShape::SplitH),
            _ => None,
        }
    }

    /// Gets the directions this mirror/splitter will redirect the beam.
    /// `incoming` is the direction relative to the object
    /// (e.g. `Direction::North` means southward beam coming in from the north of the object).
    pub fn redirections(&self, incoming: Direction) -> Vec<Direction> {
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
            SplitV => match incoming {
                N => vec![W, E],
                S => vec![W, E],
                W => vec![],
                E => vec![],
            },
            SplitH => match incoming {
                N => vec![],
                S => vec![],
                W => vec![N, S],
                E => vec![N, S],
            },
        }
    }
}

impl Display for MirrorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Debug for MirrorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
