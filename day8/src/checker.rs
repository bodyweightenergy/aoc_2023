use super::Address;

use std::collections::HashMap;

pub struct StepChecker {
    map: HashMap<Address, Vec<usize>>,
}

impl StepChecker {
    pub fn new(workers: &[Address]) -> Self {
        let mut map = HashMap::new();
        for w in workers {
            map.insert(w.clone(), Vec::new());
        }
        Self { map }
    }

    pub fn check(&mut self, worker: &Address, steps: usize) -> bool {
        // Update this worker
        self.map.get_mut(worker).unwrap().push(steps);

        #[cfg(debug)]
        {
            println!("New Val = #{worker} <= {steps}");
            for (k, v) in &self.map {
                println!("#{k} = {:?}", v);
            }
        }

        // Check is latest value exists in all worker values
        if self.map.values().all(|vec| vec.contains(&steps)) {
            // ALL WORKERS HAVE THIS STEPS VALUE!!
            println!("Complete match found @ {steps}!!");
            return true;
        }

        // Remove any values between new steps value, and previous steps value
        // Since these values are known to have been skipped on at least one worker,
        // and therefore are guarenteed to not work.
        // let worker_vals = self.map.get(&worker).unwrap();
        // if worker_vals.len() > 1 {
        //     let prev_steps_val = worker_vals[worker_vals.len() - 2];
        //     // println!("Deleting all values between ({prev_steps_val} - {steps})");
        //     let exclusion_range = (prev_steps_val + 1..steps);
        //     for (k, v) in self.map.iter_mut() {
        //         v.retain(|e| !exclusion_range.contains(e));
        //         // println!("#{k} = {v:?}");
        //     }
        // }

        let mut removed = false;
        // Remove earliest elements if they are smaller than other vector earliest elements
        // Since other vectors can't go back in time, anything before their earliest elements is invalid
        let earliest_steps: Vec<usize> = self
            .map
            .values()
            .filter(|vec| !vec.is_empty())
            .map(|vec| *vec.first().unwrap())
            .collect();
        for (k, v) in self.map.iter_mut() {
            while v.len() > 1 && earliest_steps.iter().any(|e| *e > v[0]) {
                v.remove(0);
                removed = true;
            }
        }

        #[cfg(debug)]
        if removed {
            println!("Removed values:");
            for (k, v) in &self.map {
                println!("#{k} = {v:?}");
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn simple() {
        let ws = vec!["AAA".into(), "BBB".into(), "CCC".into()];
        let mut checker = StepChecker::new(&ws);

        for i in 0..10 {
            let addr = &ws[i % 3];
            checker.check(addr, i * 2);
        }
    }
}
