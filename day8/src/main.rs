use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
    sync::{
        mpsc::{},
        
    },
    thread::{self, current},
    time::{Instant},
};

use crossbeam_queue::SegQueue;
use humansize::{format_size, DECIMAL};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tools::Opt;

use crate::checker::StepChecker;

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

    let start_nodes: Vec<Address> = nodes
        .iter()
        .filter(|n| n.address.0[2] == 'A')
        .map(|n| n.address.clone())
        .collect();
    println!("Starting Nodes ({}) = {:?}", start_nodes.len(), start_nodes);

    let start_time = Instant::now();

    let mut workers = Vec::<worker::Worker>::new();

    for n in &start_nodes {
        let worker = worker::Worker::start(n.clone(), node_map.clone(), instructions.clone());
        workers.push(worker);
    }

    let mut checker = StepChecker::new(&start_nodes);

    let mut checker_bw_ctr = BandwidthCounter::start_new("checker");
    // 'outer: loop {
        'outer: for _ in 0..10 {
        let mut step = 0;
        for w in &workers {
            let next = w.next();
            step = next;
            if checker.check(&w.start, next) {
                println!("Eureka!! @ step {}", next);
                break 'outer;
            }
        }
        if checker_bw_ctr.check() {
            // for w in &workers {
            //     println!("#{}: {} in queue", w.start, w.end_queue.len());
            // }
            println!("Step @ {step}");
        }
    }

    for w in &workers {
        w.stop();
    }

    let end_time = Instant::now();

    // println!(
    //     "Done in {step_ctr} steps (took {:?})",
    //     (end_time - start_time)
    // );
}

struct EndResult {
    step_count: usize,
    address: Address,
}

impl Debug for EndResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.step_count, self.address)
    }
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
    name: String,
    last_time: Instant,
    last_count: usize,
}

impl BandwidthCounter {
    pub fn start_new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            last_time: Instant::now(),
            last_count: 0usize,
        }
    }

    pub fn check(&mut self) -> bool {
        self.last_count += 1;
        let now = Instant::now();
        let elapsed = now - self.last_time;

        if elapsed.as_secs() > 5 {
            let bw = (self.last_count as f64 / elapsed.as_secs_f64()) as usize;
            let f = format_size(bw, DECIMAL);
            println!("{}: {f} step/sec", self.name);
            self.last_count = 0;
            self.last_time = now;
            return true;
        }
        false
    }
}

mod worker;

mod checker;
