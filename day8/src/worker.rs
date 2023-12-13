use super::BandwidthCounter;

use std::sync::mpsc::Receiver;

use std::thread;
use std::time::Duration;

use std::sync::mpsc::channel;

use super::Instruction;

use super::Node;

use std::collections::HashMap;

use super::EndResult;

use crossbeam_queue::ArrayQueue;
use crossbeam_queue::SegQueue;

use std::sync::Arc;

use std::sync::mpsc::Sender;

use std::thread::JoinHandle;

use super::Address;

pub struct Worker {
    pub start: Address,
    thread: JoinHandle<()>,
    stop_sender: Sender<()>,
    pub end_queue: Arc<ArrayQueue<usize>>,
}

impl Worker {
    pub fn start(
        start_node: Address,
        node_map: HashMap<Address, Node>,
        instructions: Vec<Instruction>,
    ) -> Self {
        let (tx, rx) = channel();
        let queue = Arc::new(ArrayQueue::new(2000));
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

    /// Get next result
    pub fn next(&self) -> usize {
        loop {
            if let Some(r) = self.end_queue.pop() {
                return r;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub(crate) fn run(
        node_map: HashMap<Address, Node>,
        instructions: Vec<Instruction>,
        start: Address,
        end_queue: Arc<ArrayQueue<usize>>,
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
                    end_queue.push(step_ctr);
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
