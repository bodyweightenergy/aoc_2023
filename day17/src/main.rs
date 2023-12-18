use std::fmt::Display;

use pathfinding::directed::dijkstra::dijkstra;
use tools::{Arena, Direction, Opt, Position};

fn main() {
    let opt = Opt::load();
    let input = opt.input().lines();
    let lines = opt.lines();

    let data: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let arena = Arena::new(data);
    let darena = DArena { arena };

    let start = Position::new(0, 0);
    let goal = Position::new(darena.arena.width() - 1, darena.arena.height() - 1);

    let result = dijkstra(&start, |p| darena.successors(p), |p| *p == goal);

    if let Some((path, score)) = result {
        let min_heat_loss = score;
        darena.arena.print(|pos, val| if path.contains(pos) { '#' } else { '.' });

        println!("Heat Loss = {min_heat_loss}");
    }
}

struct DArena {
    arena: Arena<u8>,

}

impl DArena {
    pub fn successors(&self, start: &Position) -> Vec<(Position, u8)> {
        // Run local dijkstra up to 4th place neighbor
        let neighbors = self.arena.get_neighbors(start);
    }
}
