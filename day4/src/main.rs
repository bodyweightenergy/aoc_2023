use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Debug, Display},
};

use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    let lines: Vec<&str> = input.lines().collect();

    let mut state = Problem::new();
    state.populate_cards(&lines.iter().map(|l| Card::new(l)).collect::<Vec<_>>());
    state.total_winnings();

    // for (i, t) in &state.copies {
    //     println!("Card [{}] = {t:?}", i);
    // }

    println!(
        "Total = {}",
        state.copies.iter().sum::<usize>()
    );
}

struct Problem {
    cards: Vec<Card>,
    copies: Vec<usize>,
}

impl Problem {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            copies: Vec::new(),
        }
    }

    pub fn populate_cards(&mut self, cards: &[Card]) {
        self.cards.extend_from_slice(cards);
        self.copies = vec![1; cards.len()];
    }

    /// Scans cards for their ultimate totals
    pub fn total_winnings(&mut self) {
        // for i in 0..self.cards.len() {
        //     self.winnings(i + 1);
        // }

        // assert_eq!(self.copies.len(), self.cards.len());

        for (i, c) in self.cards.iter().enumerate() {
            let score = c.points;

            if score == 0 {
                continue;
            }

            let times = self.copies[i];

            for copy in &mut self.copies[(i + 1)..=(i + score)] {
                *copy += times;
            }
        }
    }

    /// Scans a single card, and walks through all winnings
    pub fn winnings(&mut self, card_idx: usize) -> usize {
        let card = self.cards[card_idx - 1].points;

        // Check if already calculated

        let won_card_range = card_idx + 1..self.cards.len().min(card_idx + 1 + card);
        println!("[{card_idx}] + {} -> {won_card_range:?}", card);
        // Start by counting self
        let mut won_cards = 1;
        for i in won_card_range.clone() {
            // Count won cards
            won_cards += self.winnings(i);
        }
        println!("[{card_idx}] = {won_card_range:?} ({})", won_cards);
        self.copies.insert(card_idx, won_cards);
        won_cards
    }
}

// fn process(
//     map: &mut HashMap<usize, Card>,
//     total_map: &mut HashMap<usize, usize>,
//     total_pts: &mut usize,
//     prev_pts: &mut usize,
//     card_idx: usize,
// ) {
//     if let Some(total) = total_map.get(&card_idx) {
//         *total_pts += total;
//         return;
//     }
//     if !map.contains_key(&card_idx) {
//         return;
//     }
//     // println!("Process({prev_pts}): {card_idx}");
//     *prev_pts += 1;
//     let curr_pts = map
//         .get(&card_idx)
//         .expect(&format!("key {card_idx} not found"))
//         .points;
//     if curr_pts > 0 {
//         for i in card_idx + 1..card_idx + 1 + curr_pts {
//             process(map, total_map, total_pts, prev_pts, i)
//         }
//     } else {
//         total_map.insert(card_idx, *prev_pts);
//         *prev_pts = 0;
//     }
// }

#[derive(Debug, Clone)]
struct Card {
    points: usize,
}

impl Card {
    pub fn new(line: &str) -> Self {
        let vert_parts: Vec<&str> = line.split('|').collect();

        let wins: Vec<usize> = vert_parts[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .skip(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let mine: Vec<usize> = vert_parts[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let points = Self::points(wins, mine);

        Self { points }
    }

    pub fn points(wins: Vec<usize>, mine: Vec<usize>) -> usize {
        let num = mine.iter().filter(|n| wins.contains(n)).count();
        // let pts = if num > 0 { 1 << (num - 1) } else { 0 };
        // println!("Num of winning cards = {num}, Points = {pts}");
        num
    }
}
