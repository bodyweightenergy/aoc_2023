use std::collections::HashMap;

use itertools::Itertools;
use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    let lines = input.lines();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

// #[derive(Debug)]
// struct Hand {
//     cards: [Card; 5],
// }

#[derive(Debug)]
enum Hand {
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

impl Hand {
    pub fn new(input_str: &str) -> Self {
        assert!(input_str.len() == 5);

        let cards: Vec<Card> = input_str.chars().map(|c| Card::from(c)).collect();

        let sorted = Hand::sort_cards(&cards);

        // Check FiveOfKind
        if sorted.len() == 1 {
            Hand::FiveOfKind { label: sorted[0].0 }
        } else if sorted.len() == 2 {
            // Four of Kind
            if sorted[0].1 == 4 {
                Hand::FourOfKind {
                    label: sorted[0].0,
                    remainder: sorted[1].0,
                }
            } else {
                Hand::FullHouse {
                    primary: sorted[0].0,
                    secondary: sorted[1].0,
                }
            }
        }
        // Three Of Kind
        else if sorted.len() == 3 {
            if sorted[0].1 == 3 {
                Hand::ThreeOfKind {
                    label: sorted[0].0,
                    remainder: [sorted[1].0, sorted[2].0],
                }
            } else {
                // Two Pair
                Hand::TwoPair {
                    first_pair: sorted[0].0,
                    second_pair: sorted[1].0,
                    remainder: sorted[2].0,
                }
            }
        }
        // One Pair
        else if sorted.len() == 4 {
            Hand::OnePair {
                pair: sorted[0].0,
                remainder: [sorted[1].0, sorted[2].0, sorted[3].0],
            }
        }
        // High Card
        else {
            Hand::HighCard([cards[0], cards[1], cards[2], cards[3], cards[4]])
        }
    }

    /// Gets the rank of this hand
    pub fn rank(&self) -> usize {
        match self {
            Hand::FiveOfKind { label } => 7,
            Hand::FourOfKind { label, remainder } => 6,
            Hand::FullHouse { primary, secondary } => 5,
            Hand::ThreeOfKind { label, remainder } => 4,
            Hand::TwoPair { first_pair, second_pair, remainder } => 3,
            Hand::OnePair { pair, remainder } => 2,
            Hand::HighCard(_) => 1,
        }
    }

    fn sort_cards(stack: &[Card]) -> Vec<(Card, usize)> {
        let mut unsorted: HashMap<Card, usize> = HashMap::new();

        for card in stack {
            unsorted.entry(*card).and_modify(|v| *v += 1).or_insert(1);
        }

        let sorted = unsorted
            .into_iter()
            .sorted_by(|a, b| a.1.cmp(&b.1))
            .collect::<Vec<(Card, usize)>>();

        sorted
    }
}
