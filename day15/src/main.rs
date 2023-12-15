use std::{
    collections::HashMap,
    hash::{BuildHasher, Hasher},
};

use itertools::Itertools;
use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();

    let seq_items: Vec<&str> = input
        .split(&[',', '\r', '\n'])
        .filter(|s| !s.is_empty())
        .collect();
    let operations: Vec<Operation> = seq_items.iter().map(|s| Operation::new(s)).collect();

    let mut boxes: HashMap<u8, LensBox> = HashMap::new();
    for n in 0..=255u8 {
        boxes.insert(n, LensBox::new());
    }

    for op in operations {
        let label_hash = hash(&op.label);
        let box_entry = boxes.entry(label_hash);
        // Add
        if let Some(focal) = op.focal {
            box_entry.and_modify(|b| b.add_lens(&op.label, focal));
        }
        // Remove
        else {
            box_entry.and_modify(|b| b.remove_lens(&op.label));
        }
    }

    let mut total_power = 0;
    for i in 0..=255u8 {
        let b = &boxes[&i];
        if b.lenses.len() > 0 {
            let lens_str = b.lenses.iter().fold(String::new(), |mut acc, b| {
                acc.push_str(&format!("[{} {}] ", b.label, b.focal));
                acc
            });
            let box_focal_power = b.focal_power() * (i as usize + 1);
            println!("Box {i} ({}): {}", box_focal_power,lens_str);
            total_power += box_focal_power;
        }
    }

    println!("Total Power = {total_power}");

}

struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    pub fn new() -> Self {
        Self { lenses: vec![] }
    }

    pub fn add_lens(&mut self, label: &str, focal: usize) {
        for lens in &mut self.lenses {
            if lens.label == label {
                lens.focal = focal;
                return;
            }
        }
        self.lenses.push(Lens::new(label.to_owned(), focal));
    }

    pub fn remove_lens(&mut self, label: &str) {
        if let Some(lens_pos) = self.lenses.iter().position(|l| l.label == label) {
            self.lenses.remove(lens_pos);
        }
    }

    pub fn focal_power(&self) -> usize {
        self.lenses.iter().enumerate().map(|(i, l)| (i + 1) * l.focal).sum()
    }
}

struct Lens {
    label: String,
    focal: usize,
}

impl Lens {
    pub fn new(label: String, focal: usize) -> Self {
        Self {
            label,
            focal,
        }
    }
}

/// Hash a string using the LabelHasher
fn hash(input: &str) -> u8 {
    let mut hasher = LabelHasher::new();
    hasher.write(input.as_bytes());
    hasher.finish() as u8
}

struct LabelHasher {
    state: u64,
}

impl LabelHasher {
    pub fn new() -> Self {
        Self { state: 0 }
    }
}

impl Hasher for LabelHasher {
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.state += *b as u64;
            self.state *= 17;
            self.state %= 256;
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

#[derive(Debug)]
struct Operation {
    label: String,
    focal: Option<usize>,
}

impl Operation {
    pub fn new(input: &str) -> Operation {
        if input.contains('=') {
            let mut parts = input.split('=');
            Operation {
                label: parts.next().unwrap().to_owned(),
                focal: Some(parts.next().unwrap().parse().unwrap()),
            }
        } else {
            Operation {
                label: input.replace("-", ""),
                focal: None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn hash_test() {
        let input = "HASH";
        let res = hash(input);

        assert_eq!(res, 52u8);
    }

    #[test]
    pub fn example_labels() {
        let example = include_str!("../example.txt");
        let operations: Vec<Operation> = example.split(',').map(|s| Operation::new(s)).collect();

        for op in operations {
            println!("{} => {}", op.label, hash(&op.label));
        }
    }
}
