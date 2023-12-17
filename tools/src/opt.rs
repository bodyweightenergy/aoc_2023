use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2023_day5", about = "Advent of Code 2023 - Day 5 Solution")]
pub struct Opt {
    /// Enable part1 input parsing
    #[structopt(long = "part1")]
    pub is_part1: bool,

    /// Input file path
    #[structopt(long = "example")]
    pub is_example: bool,
}

impl Opt {
    pub fn load() -> Self {
        Self::from_args()
    }

    pub fn file(&self) -> PathBuf {
        if self.is_example {
            PathBuf::from("example.txt")
        } else {
            PathBuf::from("input.txt")
        }
    }

    pub fn input(&self) -> String {
        std::fs::read_to_string(self.file()).unwrap()
    }

    pub fn lines(&self) -> Vec<String> {
        self.input().lines().map(|l| l.to_owned()).collect()
    }
}
