use super::*;
use std::collections::{HashMap, HashSet};

pub struct Arena {
    pub pipe_map: HashMap<Position, Pipe>,
    pub ground_map: HashSet<Position>,
    pub start_pipe_pos: Position,
    pub max_width: usize,
    pub max_height: usize,
}

impl Arena {
    pub fn new(lines: Vec<&str>) -> Self {
        let mut pipe_map = HashMap::new();
        let mut ground_map = HashSet::new();
        let mut start_pos = None;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Position { x, y };
                // Start tile
                if c == 'S' {
                    start_pos = Some(position.clone());
                } else if c == '.' {
                    ground_map.insert(position);
                } else if let Some(pipe_type) = PipeShape::from_char(c) {
                    let pipe = Pipe {
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

        let mut arena = Arena {
            pipe_map,
            ground_map,
            start_pipe_pos: start_pos.unwrap(),
            max_width,
            max_height,
        };

        let start_pipe = arena.start_pipe();
        arena
            .pipe_map
            .insert(arena.start_pipe_pos.clone(), start_pipe);

        arena
    }

    /// Gets a neighbor tile, if it exists.
    pub fn get_adjacent(&self, position: &Position, direction: Direction) -> Option<Position> {
        let tile_pos = &position;
        let x = tile_pos.x;
        let y = tile_pos.y;
        let adj_pos = match direction {
            Direction::N => {
                if y == 0 {
                    return None;
                } else {
                    Position { x, y: y - 1 }
                }
            }
            Direction::S => {
                if y == self.max_height {
                    return None;
                } else {
                    Position { x, y: y + 1 }
                }
            }
            Direction::W => {
                if x == 0 {
                    return None;
                } else {
                    Position {
                        x: tile_pos.x - 1,
                        y,
                    }
                }
            }
            Direction::E => {
                if x == self.max_width {
                    return None;
                } else {
                    Position {
                        x: tile_pos.x + 1,
                        y,
                    }
                }
            }
        };
        Some(adj_pos)
    }

    pub fn start_pipe(&self) -> Pipe {
        let adjacents = self.get_adjacents(&self.start_pipe_pos);
        let adjacent_tiles: Vec<(Direction, PipeShape)> = adjacents
            .iter()
            .filter_map(|(dir, pos)| {
                if let Some(tile) = self.pipe_map.get(pos) {
                    Some((*dir, tile.pipe_type))
                } else {
                    None
                }
            })
            .collect();

        // Adjacent tile type is inverted (e.g. adjacent west connects to start tile east)
        let mut valid_dirs = vec![];
        for (adj_dir, adj_type) in adjacent_tiles {
            match adj_dir {
                Direction::N => match adj_type {
                    PipeShape::V | PipeShape::SE | PipeShape::SW => valid_dirs.push(adj_dir),
                    _ => {}
                },
                Direction::S => match adj_type {
                    PipeShape::V | PipeShape::NE | PipeShape::NW => valid_dirs.push(adj_dir),
                    _ => {}
                },
                Direction::W => match adj_type {
                    PipeShape::H | PipeShape::NE | PipeShape::SE => valid_dirs.push(adj_dir),
                    _ => {}
                },
                Direction::E => match adj_type {
                    PipeShape::H | PipeShape::NW | PipeShape::SW => valid_dirs.push(adj_dir),
                    _ => {}
                },
            }
        }

        // Determine start tile type by checking connecting neighbor directions
        let start_type = if valid_dirs.contains(&Direction::N) {
            if valid_dirs.contains(&Direction::W) {
                PipeShape::NW
            } else if valid_dirs.contains(&Direction::E) {
                PipeShape::NE
            } else {
                PipeShape::V
            }
        } else if valid_dirs.contains(&Direction::S) {
            if valid_dirs.contains(&Direction::W) {
                PipeShape::SW
            } else {
                PipeShape::SE
            }
        } else {
            PipeShape::H
        };

        Pipe {
            position: self.start_pipe_pos.clone(),
            pipe_type: start_type,
        }
    }

    pub fn get_adjacents(&self, position: &Position) -> HashMap<Direction, Position> {
        let mut adjacents = HashMap::new();
        for dir in Direction::all() {
            if let Some(neighbor) = self.get_adjacent(position, dir) {
                adjacents.insert(dir, neighbor);
            }
        }

        adjacents
    }

    /// Gets all neighbors of the selected tile.
    pub fn get_neighbors(&self, tile: &Pipe) -> HashMap<Direction, &Pipe> {
        let mut adjacents = HashMap::new();
        for dir in Direction::all() {
            if let Some(neighbor) = self.get_adjacent(&tile.position, dir) {
                adjacents.insert(dir, neighbor);
            }
        }
        let valid_dirs = tile.pipe_type.directions();
        for dir in Direction::all() {
            if !valid_dirs.contains(&dir) {
                adjacents.remove(&dir);
            }
        }

        let mut neighbors = HashMap::new();
        for (dir, pos) in adjacents {
            if let Some(neighbor) = self.pipe_map.get(&pos) {
                neighbors.insert(dir, neighbor);
            }
        }
        neighbors
    }

    pub fn get_tile(&self, pos: &Position) -> Option<&Pipe> {
        self.pipe_map.get(&pos)
    }

    pub fn print(&self) {
        let mut output = String::new();
        for y in 0..self.max_height {
            for x in 0..self.max_width {
                let pos = Position { x, y };
                if pos == self.start_pipe_pos {
                    output.push('S');
                } else if let Some(tile) = self.pipe_map.get(&pos) {
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
                    if pos == self.start_pipe_pos {
                        output.push('S');
                    } else if let Some(tile) = self.pipe_map.get(&pos) {
                        output.push(tile.pipe_type.to_char());
                    } else {
                        output.push('.');
                    }
                }
                else {
                    output.push(' ');
                }
            }
            output.push_str("\r\n");
        }

        println!("{}", output);
    }
}

#[derive(Debug, Clone)]
pub struct Pipe {
    /// Tile pipe shape
    pub pipe_type: PipeShape,
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
pub enum PipeShape {
    V,
    H,
    NE,
    NW,
    SW,
    SE,
}

impl PipeShape {
    pub fn to_char(&self) -> char {
        match self {
            PipeShape::V => '|',
            PipeShape::H => '-',
            PipeShape::NE => 'L',
            PipeShape::NW => 'J',
            PipeShape::SW => '7',
            PipeShape::SE => 'F',
        }
    }

    pub fn from_char(input: char) -> Option<PipeShape> {
        match input {
            '|' => Some(PipeShape::V),
            '-' => Some(PipeShape::H),
            'L' => Some(PipeShape::NE),
            'J' => Some(PipeShape::NW),
            '7' => Some(PipeShape::SW),
            'F' => Some(PipeShape::SE),
            _ => None,
        }
    }

    /// Gets the directions this pipe type opens towards.
    pub fn directions(&self) -> Vec<Direction> {
        match self {
            PipeShape::V => vec![Direction::N, Direction::S],
            PipeShape::H => vec![Direction::W, Direction::E],
            PipeShape::NE => vec![Direction::N, Direction::E],
            PipeShape::NW => vec![Direction::N, Direction::W],
            PipeShape::SW => vec![Direction::S, Direction::W],
            PipeShape::SE => vec![Direction::S, Direction::E],
        }
    }
}

impl Display for PipeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Debug for PipeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn neighbors() {
        let example = include_str!("../example.txt");
        let arena = arena::Arena::new(example.lines().collect());

        let tile = arena.get_tile(&Position::new(1, 3)).unwrap();

        let neighbors = arena.get_neighbors(&tile);
        assert!(neighbors.contains_key(&Direction::N));
        assert!(neighbors.contains_key(&Direction::E));

        let start_tile = arena.start_pipe();
        assert_eq!(start_tile.position, Position::new(1, 1));
        assert_eq!(start_tile.pipe_type, PipeShape::SE);
    }
}
