use std::fmt::{Debug, Display};

use intbits::Bits;
use itertools::Itertools;
use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    let lines: Vec<&str> = input.lines().collect();

    let rows: Vec<Row> = lines.iter().map(|l| Row::new(l)).collect();

    let mut cnt = 0;

    for row in rows {
        let a = row.get_possible_arrangements();
        cnt += a.len();
    }

    println!("Total count = {cnt}");
}

struct Row {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Row {
    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        let mut springs: Vec<Spring> = parts[0].chars().map(|c| Spring::from_char(c)).collect();
        let groups: Vec<usize> = parts[1]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        springs.push(Spring::Unknown);
        let unfolded_springs = (0..5).map(|_| springs.clone()).concat();
        let unfolded_groups = (0..5).map(|_| groups.clone()).concat();

        Row {
            springs: unfolded_springs,
            groups: unfolded_groups,
        }
    }

    /// Calculates the bad spring group sizes.
    pub fn get_groups(springs: &[Spring]) -> Vec<usize> {
        let mut groups = vec![];
        let mut start_idx: Option<usize> = None;
        for (i, s) in springs.iter().chain(Some(&Spring::Good)).enumerate() {
            if *s == Spring::Bad {
                if start_idx.is_none() {
                    start_idx = Some(i);
                }
            } else {
                if let Some(idx) = start_idx {
                    groups.push(i - idx);
                    start_idx = None;
                }
            }
        }



        groups
    }

    /// Calculates all possible known arrangements.
    pub fn get_possible_arrangements(&self) -> Vec<Vec<Spring>> {
        let known_springs = [Spring::Good, Spring::Bad];
        let mut good_spring_arrangements = vec![];

        // println!("{:?} => {:?}", self.springs, self.groups);
        let number_of_unknowns = self
            .springs
            .iter()
            .filter(|s| **s == Spring::Unknown)
            .collect::<Vec<_>>()
            .len();
        let perms: Vec<Vec<Spring>> = get_spring_combos(number_of_unknowns);
        let unk_positions: Vec<usize> = self
            .springs
            .iter()
            .positions(|s| *s == Spring::Unknown)
            .collect();

        // println!("Unk pos = {:?}", unk_positions);
        // println!("Perms ({})= {:?}", perms.len(), perms);

        for perm in &perms {
            // Create temporary springs replaced with permuatation
            let mut working_springs = self.springs.clone();

            for (s, i) in perm.iter().zip(unk_positions.iter()) {
                working_springs[*i] = s.clone();
            }

            // Test new springs to see if fits groupings
            let working_groups = Self::get_groups(&working_springs);
            if working_groups == self.groups {
                // println!("found perm: {working_springs:?} => {:?}", working_groups);
                good_spring_arrangements.push(working_springs);
            }
        }

        // println!("");
        good_spring_arrangements
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Spring {
    Good,
    Bad,
    Unknown,
}

impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Spring {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Spring::Good,
            '#' => Spring::Bad,
            '?' => Spring::Unknown,
            _ => panic!("Invalid spring character: {c}"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Spring::Good => '.',
            Spring::Bad => '#',
            Spring::Unknown => '?',
        }
    }
}

fn get_spring_combos(k: usize) -> Vec<Vec<Spring>> {
    let largest_binary = 0b1usize << k;

    let mut combos = vec![];
    for n in 0..largest_binary {
        let springs: Vec<Spring> = (0..k)
            .map(|i| if n.bit(i) { Spring::Good } else { Spring::Bad })
            .collect();
        combos.push(springs);
    }

    combos
}
