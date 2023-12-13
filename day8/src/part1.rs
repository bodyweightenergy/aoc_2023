use std::{collections::HashMap, fmt::Display, time::Instant};

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

    let mut node_map = HashMap::<Address, Node>::new();
    for node in &nodes {
        node_map.insert(node.address.clone(), node.clone());
    }

    let start_node = node_map.get(&"AAA".into()).unwrap();

    let mut step_ctr = 0usize;
    let mut current_node = start_node;

    let start_time = Instant::now();

    'outer: loop {
        for instruction in &instructions {
            let next_node_addr = match instruction {
                Instruction::Right => current_node.right.clone(),
                Instruction::Left => current_node.left.clone(),
            };

            let next_node = node_map.get(&next_node_addr).unwrap();
            // println!("{} + {} => {}", current_node, instruction, next_node);
            step_ctr += 1;

            current_node = next_node;

            if current_node.address == "ZZZ".into() {
                break 'outer;
            }
        }
    }

    let end_time = Instant::now();

    println!("Done in {step_ctr} steps (took {:?})", (end_time - start_time));
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
