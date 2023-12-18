pub struct Arena<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Arena<T> 
where T: Clone {
    pub fn new(input: Vec<Vec<T>>) -> Self {
        let height = input.len();
        let width = input[0].len();
        Self {
            data: input,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, pos: &Position) -> &T {
        &self.data[pos.y][pos.x]
    }

    pub fn get_mut(&mut self, pos: &Position) -> &mut T {
        &mut self.data[pos.y][pos.x]
    }

    pub fn adjacent_dirs(&self, pos: &Position) -> Vec<Direction> {
        let mut dirs = vec![];
        if pos.x > 0 {
            dirs.push(Direction::W);
        }
        if pos.x < self.width - 1 {
            dirs.push(Direction::E);
        }
        if pos.y > 0 {
            dirs.push(Direction::N);
        }
        if pos.y < self.height - 1 {
            dirs.push(Direction::S);
        }
        dirs
    }

    pub fn get_adjacent(&self, pos: &Position, dir: Direction) -> Option<Position> {
        let valid_dirs = self.adjacent_dirs(pos);
        if valid_dirs.contains(&dir) {
            let adj_pos = match dir {
                Direction::N => Position::new(pos.x, pos.y - 1),
                Direction::S => Position::new(pos.x, pos.y + 1),
                Direction::W => Position::new(pos.x - 1, pos.y),
                Direction::E => Position::new(pos.x + 1, pos.y),
            };

            Some(adj_pos)
        }
        else {
            None
        }
    }

    pub fn print<F>(&self, f: F) where F: Fn(&Position, &T) -> char {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x, y);
                let c = f(&pos, self.get(&pos));
                s.push(c);
            }
            s.push('\n');
        }
        println!("{}", s);
    }
}


impl<T> Arena<T> 
where T: Clone {
    pub fn get_neighbors(&self, pos: &Position) -> Vec<(Position, T)> {
        let mut neighbors: Vec<(Position, T)> = vec![];
        for dir in Direction::all() {
            if let Some(adjacent) = self.get_adjacent(pos, dir) {
                let val = self.get(&adjacent).clone();
                neighbors.push((adjacent, val));
            }
        }

        neighbors
    }
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

use std::fmt::{Debug, Display};

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
