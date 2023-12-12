use std::collections::HashMap;

use tools::Opt;

fn main() {
    let opt = Opt::load();

    let input = opt.input();
    let mut lines = input.lines();
    if opt.is_part1 {
        let time_line = lines.next().unwrap();
        let distance_line = lines.next().unwrap();

        let times: Vec<u64> = time_line
            .split(&[' '])
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let distances: Vec<u64> = distance_line
            .split(&[' '])
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let races: Vec<Race> = times
            .iter()
            .zip(distances.iter())
            .map(|(t, d)| Race {
                time: *t,
                record_distance: *d,
            })
            .collect();

        println!("Races = {:#?}", races);

        let mut runs = 1;
        for race in races {
            let sufficient_runs = dbg!(race.get_sufficient_runs());
            runs *= sufficient_runs.len();
        }

        println!("Runs = {runs}");
    }
    // Part 2
    else {
        let time_line = lines.next().unwrap();
        let distance_line = lines.next().unwrap();

        let time: u64 = time_line
            .split(&[' '])
            .filter(|s| !s.is_empty())
            .skip(1)
            .fold(String::new(), |mut a, b| {
                a.push_str(b);
                a
            })
            .parse::<u64>()
            .unwrap();
        let distance_record: u64 = distance_line
            .split(&[' '])
            .filter(|s| !s.is_empty())
            .skip(1)
            .fold(String::new(), |mut a, b| {
                a.push_str(b);
                a
            })
            .parse::<u64>()
            .unwrap();

        let race = Race { time, record_distance: distance_record };
        let runs = race.get_sufficient_runs().len();

        println!("Runs = {runs}");
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    /// Runs the race with the selected button press time.
    pub fn run(&self, button_time: u64) -> u64 {
        let speed = button_time;
        let remain_time = self.time - button_time;
        let distance = remain_time * speed;
        distance
    }

    /// Finds the optimal button press time to win the race.
    pub fn get_sufficient_runs(&self) -> Vec<u64> {
        let mut runs: HashMap<u64, u64> = HashMap::new();
        for button_time in 0..self.time {
            let dist = self.run(button_time);
            runs.insert(button_time, dist);
        }

        let max_button_presses = runs
            .iter()
            .filter(|(_, v)| **v > self.record_distance)
            .map(|(k, _)| *k)
            .collect();

        max_button_presses
    }
}
