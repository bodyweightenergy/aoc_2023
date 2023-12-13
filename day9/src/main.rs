use std::collections::{BTreeMap, VecDeque};

use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    let lines: Vec<&str> = input.lines().collect();

    let mut acc_score = 0;
    for line in &lines {
        let mut seq = Sequence::new(line);
        seq.print();

        while let Some(idx) = seq.calc_next_level() {
            // seq.print();
        }

        let res = if opt.is_part1 {
            seq.calc_next_val()
        } else {
            seq.calc_prev_val()
        };

        println!("New Value: {res}");
        seq.print();
        acc_score += res;
    }

    println!("Total score = {acc_score}");
}
#[derive(Debug)]
struct Sequence {
    map: BTreeMap<usize, Vec<i32>>,
}

impl Sequence {
    pub fn new(input: &str) -> Self {
        let top = input
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut map = BTreeMap::new();
        map.insert(0, top);

        Self { map }
    }

    pub fn calc_next_level(&mut self) -> Option<usize> {
        let mut next_level = vec![];

        let (bottom_idx, bottom_vals) = self.map.last_key_value().unwrap();

        if bottom_vals.iter().all(|v| *v == 0) {
            return None;
        }

        for (a, b) in bottom_vals.windows(2).map(|w| (w[0], w[1])) {
            let n = b - a;
            next_level.push(n);
        }

        let new_bottom_idx = bottom_idx + 1;
        self.map.insert(new_bottom_idx, next_level);

        Some(new_bottom_idx)
    }

    pub fn calc_next_val(&mut self) -> i32 {
        {
            let (last_layer_idx, last_layer_values) = self.map.last_key_value().unwrap();

            if !last_layer_values.iter().all(|v| *v == 0) {
                panic!("Last layer is not all zeroes!");
            }
        }

        let last_layer_idx = self.map.last_key_value().unwrap().0.clone();

        // Add zero to last layer
        self.map.get_mut(&last_layer_idx).unwrap().push(0);

        let mut layer_idx = last_layer_idx - 1;
        loop {
            let layer_values = self.map.get(&layer_idx).unwrap();
            let prev_layer_values = self.map.get(&(layer_idx + 1)).unwrap();

            let new_val = layer_values.last().unwrap() + prev_layer_values.last().unwrap();
            self.map.get_mut(&layer_idx).unwrap().push(new_val);

            if layer_idx == 0 {
                break;
            }

            layer_idx -= 1;
        }

        let res = self
            .map
            .first_key_value()
            .unwrap()
            .1
            .last()
            .unwrap()
            .clone();
        res
    }

    pub fn calc_prev_val(&mut self) -> i32 {
        {
            let (last_layer_idx, last_layer_values) = self.map.last_key_value().unwrap();

            if !last_layer_values.iter().all(|v| *v == 0) {
                panic!("Last layer is not all zeroes!");
            }
        }

        let last_layer_idx = self.map.last_key_value().unwrap().0.clone();

        // Add zero to last layer
        self.map.get_mut(&last_layer_idx).unwrap().push(0);

        let mut layer_idx = last_layer_idx - 1;
        loop {
            let layer_values = self.map.get(&layer_idx).unwrap();
            let prev_layer_values = self.map.get(&(layer_idx + 1)).unwrap();

            let new_val = layer_values.first().unwrap() - prev_layer_values.first().unwrap();
            self.map.get_mut(&layer_idx).unwrap().insert(0, new_val);

            if layer_idx == 0 {
                break;
            }

            layer_idx -= 1;
        }

        let res = self.map.get(&0).unwrap().first().unwrap().clone();
        res
    }

    pub fn print(&self) {
        println!("Sequence:");
        for (k, v) in &self.map {
            println!("[{k}] = {v:?}");
        }
        println!("");
    }
}
