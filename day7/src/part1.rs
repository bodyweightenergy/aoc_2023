use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    let lines = input.lines();

    let mut games: Vec<Game> = lines
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let hand = Hand::new(parts[0]);
            let bid = parts[1].parse::<usize>().unwrap();

            Game { hand, bid }
        })
        .collect();

    println!("Games = {games:#?}");
    games.sort_by(|a, b| a.hand.cmp(&b.hand));
    let mut total_score = 0;
    for (i, game) in games.iter().enumerate() {
        let rank = i + 1;
        let score = rank * game.bid;
        total_score += score;
        println!("#{rank}: {game:?} = {score}");
    }

    println!("Total score = {total_score}");
    
    
}

#[derive(Debug)]
struct Game {
    hand: Hand,
    bid: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
    C9 = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid card character: {}", value),
        }
    }
}

impl Card {
    pub fn count_in_stack(&self, stack: &[Card]) -> usize {
        stack
            .iter()
            .filter(|c| *c == self)
            .collect::<Vec<_>>()
            .len()
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    pub fn new(input_str: &str) -> Self {
        let cards: Vec<Card> = input_str.chars().map(|c| Card::from(c)).collect();
        let hand_type = HandType::new(&cards);

        Hand { cards, hand_type }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_rank = self.hand_type.strength();
        let other_rank = other.hand_type.strength();

        if self_rank > other_rank {
            Ordering::Greater
        } else if other_rank > self_rank {
            Ordering::Less
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                if a > b {
                    return Ordering::Greater;
                } else if a < b {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

#[derive(Debug, Clone)]
enum HandType {
    /// AAAAA
    FiveOfKind { label: Card },
    /// AAAAJ
    FourOfKind { label: Card, remainder: Card },
    /// AAAJJ
    FullHouse { primary: Card, secondary: Card },
    /// AAAJK
    ThreeOfKind { label: Card, remainder: [Card; 2] },
    /// AAJJ3
    TwoPair {
        first_pair: Card,
        second_pair: Card,
        remainder: Card,
    },
    /// AA123
    OnePair { pair: Card, remainder: [Card; 3] },
    /// 12345
    HighCard([Card; 5]),
}

impl HandType {
    pub fn new(stack: &[Card]) -> Self {
        assert!(stack.len() == 5);

        let cards: Vec<Card> = stack.to_owned();

        let sorted = HandType::sort_cards(&cards);
        println!("Sorted '{stack:?}':");
        for (c, f) in &sorted {
            println!("\t{c:?}: {f}");
        }

        // Check FiveOfKind
        if sorted.len() == 1 {
            HandType::FiveOfKind { label: sorted[0].0 }
        } else if sorted.len() == 2 {
            // Four of Kind
            if sorted[0].1 == 4 {
                HandType::FourOfKind {
                    label: sorted[0].0,
                    remainder: sorted[1].0,
                }
            } else {
                HandType::FullHouse {
                    primary: sorted[0].0,
                    secondary: sorted[1].0,
                }
            }
        }
        // Three Of Kind
        else if sorted.len() == 3 {
            if sorted[0].1 == 3 {
                HandType::ThreeOfKind {
                    label: sorted[0].0,
                    remainder: [sorted[1].0, sorted[2].0],
                }
            } else {
                // Two Pair
                HandType::TwoPair {
                    first_pair: sorted[0].0,
                    second_pair: sorted[1].0,
                    remainder: sorted[2].0,
                }
            }
        }
        // One Pair
        else if sorted.len() == 4 {
            HandType::OnePair {
                pair: sorted[0].0,
                remainder: [sorted[1].0, sorted[2].0, sorted[3].0],
            }
        }
        // High Card
        else {
            HandType::HighCard([
                sorted[0].0,
                sorted[1].0,
                sorted[2].0,
                sorted[3].0,
                sorted[4].0,
            ])
        }
    }

    /// Gets the rank of this hand, only for comparison against other hands
    pub fn strength(&self) -> usize {
        match self {
            HandType::FiveOfKind { label } => 7,
            HandType::FourOfKind { label, remainder } => 6,
            HandType::FullHouse { primary, secondary } => 5,
            HandType::ThreeOfKind { label, remainder } => 4,
            HandType::TwoPair {
                first_pair,
                second_pair,
                remainder,
            } => 3,
            HandType::OnePair { pair, remainder } => 2,
            HandType::HighCard(_) => 1,
        }
    }

    fn sort_cards(stack: &[Card]) -> Vec<(Card, usize)> {
        let mut unsorted: HashMap<Card, usize> = HashMap::new();

        for card in stack {
            unsorted.entry(*card).and_modify(|v| *v += 1).or_insert(1);
        }

        let sorted = unsorted
            .into_iter()
            .sorted_by(|a, b| {
                if a.1 == b.1 {
                    b.0.cmp(&a.0)
                } else {
                    b.1.cmp(&a.1)
                }
            })
            .collect::<Vec<(Card, usize)>>();

        sorted
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FiveOfKind { label: l_label }, Self::FiveOfKind { label: r_label }) => {
                l_label == r_label
            }
            (
                Self::FourOfKind {
                    label: l_label,
                    remainder: l_remainder,
                },
                Self::FourOfKind {
                    label: r_label,
                    remainder: r_remainder,
                },
            ) => l_label == r_label && l_remainder == r_remainder,
            (
                Self::FullHouse {
                    primary: l_primary,
                    secondary: l_secondary,
                },
                Self::FullHouse {
                    primary: r_primary,
                    secondary: r_secondary,
                },
            ) => l_primary == r_primary && l_secondary == r_secondary,
            (
                Self::ThreeOfKind {
                    label: l_label,
                    remainder: l_remainder,
                },
                Self::ThreeOfKind {
                    label: r_label,
                    remainder: r_remainder,
                },
            ) => l_label == r_label && l_remainder == r_remainder,
            (
                Self::TwoPair {
                    first_pair: l_first_pair,
                    second_pair: l_second_pair,
                    remainder: l_remainder,
                },
                Self::TwoPair {
                    first_pair: r_first_pair,
                    second_pair: r_second_pair,
                    remainder: r_remainder,
                },
            ) => {
                l_first_pair == r_first_pair
                    && l_second_pair == r_second_pair
                    && l_remainder == r_remainder
            }
            (
                Self::OnePair {
                    pair: l_pair,
                    remainder: l_remainder,
                },
                Self::OnePair {
                    pair: r_pair,
                    remainder: r_remainder,
                },
            ) => l_pair == r_pair && l_remainder == r_remainder,
            (Self::HighCard(l0), Self::HighCard(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn hand_types() {
        let input = HashMap::from([
            ("22222", HandType::FiveOfKind { label: Card::C2 }),
            (
                "22223",
                HandType::FourOfKind {
                    label: Card::C2,
                    remainder: Card::C3,
                },
            ),
            (
                "22233",
                HandType::FullHouse {
                    primary: Card::C2,
                    secondary: Card::C3,
                },
            ),
            (
                "2233K",
                HandType::TwoPair {
                    first_pair: Card::C3,
                    second_pair: Card::C2,
                    remainder: Card::K,
                },
            ),
            (
                "22JQK",
                HandType::OnePair {
                    pair: Card::C2,
                    remainder: [Card::K, Card::Q, Card::J],
                },
            ),
            (
                "23456",
                HandType::HighCard([Card::C6, Card::C5, Card::C4, Card::C3, Card::C2]),
            ),
        ]);

        for (s, h) in input {
            let hand = Hand::new(s);

            assert_eq!(hand.hand_type, h);
        }
    }
}
