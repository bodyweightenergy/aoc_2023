use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use itertools::Itertools;
use tools::Opt;

use crate::walker::PipeWalker;

mod arena;
mod walker;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let arena = arena::Arena::new(lines);
    // arena.print();
    println!("Found {} tiles.", arena.pipe_map.len());

    let mut walker = PipeWalker::new(arena);

    while let Some((score, pos)) = walker.next() {
        // println!("[{score}] = {pos}");
    }

    walker.arena.print_selection(|p| walker.arena.ground_map.contains(p) || walker.score_map.values().contains(p));

    println!("Max distance = {}", walker.score_map.len() / 2);
}

