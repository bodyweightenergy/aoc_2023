use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use tools::Opt;


fn main() {
    let opt = Opt::load();
    println!("Options: file={:?}, part1={}", opt.file(), opt.is_part1);

    let input = File::open(&opt.file()).unwrap();
    let reader = BufReader::new(input);
    let input_lines: Vec<String> = reader.lines().filter_map(|f| f.ok()).collect();
    let max_x = input_lines.first().unwrap().len();
    let max_y = input_lines.len();
    println!("Max X = {max_x}, Max Y = {max_y}");

    let mut galaxies = vec![];
    let mut curr_id: u32 = 1;
    for (y, line) in input_lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let galaxy = Galaxy {
                    id: Id(curr_id),
                    position: Position { x, y },
                };
                galaxies.push(galaxy);
                curr_id += 1;
            }
        }
    }

    // println!("Start Galaxies = {:#?}", galaxies);

    let mut empty_cols = vec![];
    for x in 0..max_x {
        if !galaxies.iter().any(|g| g.position.x == x) {
            empty_cols.push(x);
        }
    }

    let mut empty_rows = vec![];
    for y in 0..max_y {
        if !galaxies.iter().any(|g| g.position.y == y) {
            empty_rows.push(y);
        }
    }

    println!("Empty X = {empty_cols:#?}");
    println!("Empty Y = {empty_rows:#?}");

    // Expand space

    let expansion_amt = 999_999;
    for galaxy in &mut galaxies {
        let empty_cols_before = empty_cols
            .iter()
            .filter(|x| galaxy.position.x > **x)
            .collect::<Vec<_>>()
            .len();
        let empty_rows_before = empty_rows
            .iter()
            .filter(|y| galaxy.position.y > **y)
            .collect::<Vec<_>>()
            .len();

        let x_expansion = if empty_cols_before > 0 {
            empty_cols_before  * expansion_amt
        } else { 0 };
        let y_expansion = if empty_rows_before > 0 {
            empty_rows_before * expansion_amt
        } else { 0 };

        galaxy
            .position
            .expand_x(x_expansion);
        galaxy
            .position
            .expand_y(y_expansion);
    }

    let combos: Vec<(&Galaxy, &Galaxy)> = galaxies.iter().tuple_combinations().collect();
    println!("Number of galaxy combos = {}", combos.len());
    let mut total_distance = 0;
    for (a, b) in &combos {
        if a.id.0 == 5 && b.id.0 == 9 {
            // println!("#5: {:?}, #9: {:?}", a, b);
        }
        let distance = a.distance(b);
        total_distance += distance;
        // println!("({:?},{:?}) = {}", a.id.0, b.id.0, distance);
    }

    println!("Total Distance = {total_distance}");
}

#[derive(Debug)]
struct Id(u32);

#[derive(Debug)]
struct Galaxy {
    id: Id,
    position: Position,
}

impl Galaxy {
    pub fn distance(&self, other: &Galaxy) -> usize {
        self.position.distance(&other.position)
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    /// Calculates the distance between this position and another.
    /// In this world, it's simply the X + Y distances.
    pub fn distance(&self, other: &Position) -> usize {
        let x_offset = usize::abs_diff(self.x, other.x);
        let y_offset = usize::abs_diff(self.y, other.y);
        x_offset + y_offset
    }

    /// Expand space in X direction
    pub fn expand_x(&mut self, amt: usize) {
        self.x += amt;
    }

    /// Expand space in Y direction
    pub fn expand_y(&mut self, amt: usize) {
        self.y += amt;
    }
}
