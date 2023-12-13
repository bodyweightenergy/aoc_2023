use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    thread::current,
    time::{Instant, Duration},
};

use humansize::{format_size, DECIMAL};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tools::Opt;

fn main() {
    let opt = Opt::load();

    let input = opt.input();
    let lines: Vec<&str> = input.lines().collect();

    let instructions: Vec<Instruction> = lines[0]
        .chars()
        .map(|c| match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!("Invalid instruction character: {c}"),
        })
        .collect();

    let nodes: Vec<Node> = lines[2..].iter().map(|line| Node::new(line)).collect();

    let node_map = {
        let mut map = HashMap::<Address, Node>::new();
        for node in &nodes {
            map.insert(node.address.clone(), node.clone());
        }
        map
    };

    let start_nodes: Vec<&Address> = nodes
        .iter()
        .filter(|n| n.address.0[2] == 'A')
        .map(|n| &n.address)
        .collect();
    println!("Starting Nodes ({}) = {:?}", start_nodes.len(), start_nodes);

    let mut step_ctr = 0usize;
    let mut current_nodes = start_nodes;

    let start_time = Instant::now();
    let mut bw_ctr = BandwidthCounter::start_new();

    'outer: loop {
        for instruction in &instructions {

            let mut z_count = 0;
            match instruction {
                Instruction::Right => {
                    for i in 0..current_nodes.len() {
                        current_nodes[i] = &node_map[current_nodes[i]].right;
                        if current_nodes[i].0[2] == 'Z' {
                            z_count += 1;
                        }
                    }
                },
                Instruction::Left => {
                    for i in 0..current_nodes.len() {
                        current_nodes[i] = &node_map[current_nodes[i]].left;
                        if current_nodes[i].0[2] == 'Z' {
                            z_count += 1;
                        }
                    }
                },
            }

            // println!("[{step_ctr}] {instruction}");
            // for (c, n) in current_nodes.iter().zip(next_nodes.iter()) {
            //     println!("\t{} => {}", c, n);
            // }
            step_ctr += 1;

            if z_count > 1 {
                // println!("[{step_ctr}] = ({z_count}) ### {:?} ###", current_nodes);
                if current_nodes.iter().all(|n| n.0[2] == 'Z') {
                    break 'outer;
                }
            }

            bw_ctr.check();
        }
    }

    let end_time = Instant::now();

    println!(
        "Done in {step_ctr} steps (took {:?})",
        (end_time - start_time)
    );
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Right,
    Left,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Instruction::Right => "R",
            Instruction::Left => "L",
        };

        write!(f, "{s}")
    }
}

#[derive(Clone)]
struct Node {
    address: Address,
    right: Address,
    left: Address,
}

impl Node {
    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line
            .split([' ', '=', '(', ')', ','])
            .filter(|s| !s.is_empty())
            .collect();

        let address = Address::new(parts[0]);
        let left = Address::new(parts[1]);
        let right = Address::new(parts[2]);

        Node {
            address,
            left,
            right,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:({}, {})", self.address, self.left, self.right)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Address([char; 3]);

impl Address {
    pub fn new(addr_str: &str) -> Self {
        assert!(addr_str.len() == 3);
        let chars: Vec<char> = addr_str.chars().collect();

        Address([chars[0], chars[1], chars[2]])
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        Address::new(value)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

struct BandwidthCounter {
    last_time: Instant,
    last_count: usize,
}

impl BandwidthCounter {
    pub fn start_new() -> Self {
        Self {
            last_time: Instant::now(),
            last_count: 0usize,
        }
    }

    pub fn check(&mut self) {
        self.last_count += 1;
        let now = Instant::now();
        let elapsed = now - self.last_time;

        if elapsed.as_secs() > 2 {
            let bw = (self.last_count as f64 / elapsed.as_secs_f64()) as usize;
            let f = format_size(bw, DECIMAL);
            println!("{f} step/sec");
            self.last_count = 0;
            self.last_time = now;
        }
    }
}