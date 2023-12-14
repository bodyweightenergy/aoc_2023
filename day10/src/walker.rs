use std::collections::BTreeMap;

use crate::arena::{Arena, Position, Direction};

pub struct PipeWalker {
    pub arena: Arena,
    current_pos: Position,
    next_direction: Direction,
    pub score_map: BTreeMap<usize, Position>,
}

impl PipeWalker {
    pub fn new(arena: Arena) -> Self {
        let current_pos = arena.start_pipe_pos.clone();
        let start_tile = arena.start_pipe();
        let dir = arena.get_neighbors(&start_tile).keys().next().unwrap().clone();
        let mut score_map = BTreeMap::new();
        score_map.insert(0usize, current_pos.clone());

        Self {
            arena,
            current_pos,
            next_direction: dir,
            score_map,
        }
    }

    /// Walks to the next pipe, and returns it's score/distance.
    pub fn next(&mut self) -> Option<(usize, Position)> {
        if (self.current_pos == self.arena.start_pipe_pos) && self.score_map.len() > 1 {
            return None;
        }
        let current_tile = self.arena.get_tile(&self.current_pos).unwrap();
        let neighbors = self.arena.get_neighbors(&current_tile);

        let next_pipe = neighbors.get(&self.next_direction).unwrap();
        let possible_next_dirs = next_pipe.pipe_type.directions();
        let next_dir = possible_next_dirs.iter().find(|d| **d != self.next_direction.reverse()).unwrap();

        let score = self.score_map.len();
        self.score_map.insert(score, next_pipe.position.clone());
        
        self.current_pos = next_pipe.position.clone();
        self.next_direction = *next_dir;

        Some((score, self.current_pos.clone()))
    }

    
}