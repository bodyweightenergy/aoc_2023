use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use itertools::Itertools;
use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let arena = Arena::new(lines);
    println!("Found {} tiles.", arena.tile_map.len());
}

struct Arena {
    tile_map: HashMap<Position, Tile>,
    start_tile_pos: Position,
    max_width: usize,
    max_height: usize,
}

impl Arena {
    pub fn new(lines: Vec<&str>) -> Self {
        let mut tile_map = HashMap::new();
        let mut start_pos = None;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Position { x, y };
                // Start tile
                if c == 'S' {
                    start_pos = Some(position.clone());
                }
                if let Some(tile_type) = TileType::from_char(c) {
                    let tile = Tile {
                        tile_type,
                        position: position.clone(),
                    };
                    tile_map.insert(position, tile);
                }
            }
        }

        let max_width = lines[0].len();
        let max_height = lines.len();

        Arena {
            tile_map,
            start_tile_pos: start_pos.unwrap(),
            max_width,
            max_height,
        }
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

    pub fn start_tile(&self) -> Tile {
        let adjacents = self.get_adjacents(&self.start_tile_pos);
        let adjacent_tiles: Vec<(Direction, TileType)> = adjacents
            .iter()
            .map(|(dir, pos)| (*dir, self.tile_map.get(pos).unwrap().tile_type))
            .collect();

        // Adjacent tile type is inverted (e.g. adjacent west connects to start tile east)
        let mut valid_dirs = vec![];
        for (adj_dir, adj_type) in adjacent_tiles {
            match adj_dir {
                Direction::N => match adj_type {
                    TileType::V | TileType::SE | TileType::SW => valid_dirs.push(adj_dir),
                    _ => {}
                },
                Direction::S => match adj_type {
                    TileType::V | TileType::NE | TileType::NW => valid_dirs.push(adj_dir),
                    _ => {}
                },
                Direction::W => match adj_type {
                    TileType::H | TileType::NE | TileType::SE => valid_dirs.push(adj_dir),
                    _ => {}
                },
                Direction::E => match adj_type {
                    TileType::H | TileType::NW | TileType::SW => valid_dirs.push(adj_dir),
                    _ => {}
                },
            }
        }

        // Determine start tile type by checking connecting neighbor directions
        let start_type = if valid_dirs.contains(&Direction::N) {
            if valid_dirs.contains(&Direction::W) {
                TileType::NW
            } else if valid_dirs.contains(&Direction::E) {
                TileType::NE
            } else {
                TileType::V
            }
        } else if valid_dirs.contains(&Direction::S) {
            if valid_dirs.contains(&Direction::W) {
                TileType::SW
            } else {
                TileType::SE
            }
        } else {
            TileType::H
        };

        Tile {
            position: self.start_tile_pos.clone(),
            tile_type: start_type,
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
    pub fn get_neighbors(&self, tile: &Tile) -> HashMap<Direction, &Tile> {
        let mut adjacents = HashMap::new();
        for dir in Direction::all() {
            if let Some(neighbor) = self.get_adjacent(&tile.position, dir) {
                adjacents.insert(dir, neighbor);
            }
        }
        let valid_dirs = tile.tile_type.directions();
        for dir in Direction::all() {
            if !valid_dirs.contains(&dir) {
                adjacents.remove(&dir);
            }
        }

        let mut neighbors = HashMap::new();
        for (dir, pos) in adjacents {
            let neighbor = self.tile_map.get(&pos).unwrap();
            neighbors.insert(dir, neighbor);
        }
        neighbors
    }
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    position: Position,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
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
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum TileType {
    V,
    H,
    NE,
    NW,
    SW,
    SE,
}

impl TileType {
    pub fn to_char(&self) -> char {
        match self {
            TileType::V => '|',
            TileType::H => '-',
            TileType::NE => 'L',
            TileType::NW => 'J',
            TileType::SW => '7',
            TileType::SE => 'F',
        }
    }

    pub fn from_char(input: char) -> Option<TileType> {
        match input {
            '|' => Some(TileType::V),
            '-' => Some(TileType::H),
            'L' => Some(TileType::NE),
            'J' => Some(TileType::NW),
            '7' => Some(TileType::SW),
            'F' => Some(TileType::SE),
            _ => None,
        }
    }

    /// Gets the directions this tile type connects to.
    pub fn directions(&self) -> Vec<Direction> {
        match self {
            TileType::V => vec![Direction::N, Direction::S],
            TileType::H => vec![Direction::W, Direction::E],
            TileType::NE => vec![Direction::N, Direction::E],
            TileType::NW => vec![Direction::N, Direction::W],
            TileType::SW => vec![Direction::S, Direction::W],
            TileType::SE => vec![Direction::S, Direction::E],
        }
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Debug for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
