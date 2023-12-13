use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread::{self, current, JoinHandle},
    time::{Duration, Instant},
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

    let mut workers = Vec::<Worker>::new();

    for n in &start_nodes {
        let worker = Worker::start(n.clone(), node_map.clone(), instructions.clone());
        workers.push(worker);
    }

    let mut checker = StepChecker::new(&start_nodes);

    let mut bw_ctr = BandwidthCounter::start_new("checker");
    'outer: loop {
        // 'outer: for _ in 0..100 {
        let mut step = 0;
        for w in &workers {
            let next = w.next();
            step = next.step_count;
            if checker.check(&w.start, next.step_count) {
                println!("Eureka!! @ step {}", next.step_count);
                break 'outer;
            }
        }
        if bw_ctr.check() {
            for w in &workers {
                println!("#{}: {} in queue", w.start, w.end_queue.len());
            }
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

        if elapsed.as_secs() > 2 {
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

struct Worker {
    start: Address,
    thread: JoinHandle<()>,
    stop_sender: Sender<()>,
    end_queue: Arc<SegQueue<EndResult>>,
}

impl Worker {
    pub fn start(
        start_node: Address,
        node_map: HashMap<Address, Node>,
        instructions: Vec<Instruction>,
    ) -> Self {
        let (tx, rx) = channel();
        let queue = Arc::new(SegQueue::new());
        let queue_c = queue.clone();
        let start_node_c = start_node.clone();

        let t = thread::spawn(move || {
            Self::run(node_map, instructions, start_node_c, queue_c, rx);
        });

        Self {
            start: start_node,
            thread: t,
            stop_sender: tx,
            end_queue: queue,
        }
    }

    /// Stops this worker thread.
    pub fn stop(&self) {
        self.stop_sender.send(()).unwrap();
    }

    /// Gets all available results.
    pub fn available_results(&self) -> Vec<EndResult> {
        let mut results = vec![];
        while let Some(r) = self.end_queue.pop() {
            results.push(r);
        }
        results
    }

    /// Get next result
    pub fn next(&self) -> EndResult {
        loop {
            if let Some(r) = self.end_queue.pop() {
                return r;
            }
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn run(
        node_map: HashMap<Address, Node>,
        instructions: Vec<Instruction>,
        start: Address,
        end_queue: Arc<SegQueue<EndResult>>,
        stop_recv: Receiver<()>,
    ) {
        let mut current_node = &start;
        let mut step_ctr = 0usize;
        let mut input_bw_ctr = BandwidthCounter::start_new(&format!("Input #{start}"));
        let mut output_bw_ctr = BandwidthCounter::start_new(&format!("Output #{start}"));
        loop {
            for instruction in &instructions {
                let mut z_found = false;
                match instruction {
                    Instruction::Right => {
                        current_node = &node_map[current_node].right;
                        if current_node.0[2] == 'Z' {
                            z_found = true;
                        }
                    }
                    Instruction::Left => {
                        current_node = &node_map[current_node].left;
                        if current_node.0[2] == 'Z' {
                            z_found = true;
                        }
                    }
                }
                step_ctr += 1;
    
                if z_found {
                    // println!("[{step_ctr}] = ({z_count}) ### {:?} ###", current_nodes);
                    end_queue.push(EndResult {
                        step_count: step_ctr,
                        address: current_node.clone(),
                    });
                    output_bw_ctr.check();
                }
    
                input_bw_ctr.check();
    
                if let Ok(()) = stop_recv.try_recv() {
                    return;
                }
    
                // Try to release CPU if working too fast
                if end_queue.len() > 1000 {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        }
    }
}

mod checker;
