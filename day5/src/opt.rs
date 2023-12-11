use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2023_day5", about = "Advent of Code 2023 - Day 5 Solution")]
pub struct Opt {
    /// Enable part1 input parsing
    #[structopt(long = "part1")]
    pub is_part1: bool,

    /// Input file path
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
}