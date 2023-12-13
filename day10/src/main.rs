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
    start_tile: Tile,
    max_width: usize,
    max_height: usize,
}

impl Arena {
    pub fn new(lines: Vec<&str>) -> Self {
        let mut tile_map = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(tile_type) = TileType::from_char(c) {
                    let position = Position { x, y };
                    let tile = Tile {
                        tile_type,
                        position: position.clone(),
                    };
                    tile_map.insert(position, tile);
                }
            }
        }

        let start_tile = tile_map
            .values()
            .find(|t| t.tile_type == TileType::Start)
            .expect("No S tile found.")
            .clone();

        let max_width = lines[0].len();
        let max_height = lines.len();

        Arena {
            tile_map,
            start_tile,
            max_width,
            max_height,
        }
    }

    /// Gets a neighbor tile, if it exists.
    pub fn get_neighbor(&self, tile: &Tile, direction: Direction) -> Option<&Tile> {
        let tile_pos = &tile.position;
        let x = tile_pos.x;
        let y = tile_pos.y;
        let neighbor_pos = match direction {
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

        self.tile_map.get(&neighbor_pos)
    }

    pub fn start_tile_type(&self) -> TileType {
        let start_tile_pos = &self.start_tile.position;
        let neighbors = self.get_neighbors(&self.start_tile);
        let keys:Vec<Direction> = neighbors.keys().copied().collect();

        if keys.contains(&Direction::N) {
            if keys.contains(&Direction::W) {
                TileType::NW
            }
            else if keys.contains(&Direction::E) {
                TileType::NE
            }
            else {
                TileType::V
            }
        }
        else if keys.contains(&Direction::S) {
            if keys.contains(&Direction::W) {
                TileType::SW
            }
            else {
                TileType::SE
            }
        }
        else {
            TileType::H
        }
    }

    /// Gets all neighbors of the selected tile.
    pub fn get_neighbors(&self, tile: &Tile) -> HashMap<Direction, &Tile> {
        let mut neighbors = HashMap::new();
        for dir in [Direction::N, Direction::S, Direction::W, Direction::E] {
            if let Some(neighbor) = self.get_neighbor(tile, dir) {
                neighbors.insert(dir, neighbor);
            }
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

#[derive(Copy, Clone, PartialEq, Eq)]
enum TileType {
    V,
    H,
    NE,
    NW,
    SW,
    SE,
    Start,
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
            TileType::Start => 'S',
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
            'S' => Some(TileType::Start),
            _ => None,
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
