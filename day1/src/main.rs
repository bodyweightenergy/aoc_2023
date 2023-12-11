use std::collections::{HashMap, hash_map};

use tools::Opt;

fn main() {
    let opt = Opt::load();

    let input = opt.input();
    let lines = input.lines();

    if opt.is_part1 {
        let total: u32 = lines
            .map(|line| {
                let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
                let cal_value = digits.first().unwrap() * 10 + digits.last().unwrap();
                cal_value
            })
            .sum();

        println!("Total = {}", total);
    }
    // part 2
    else {
        let num_dict = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        let num_words: Vec<&&str> = num_dict.keys().collect();

        let mut total = 0;
        for line in lines {
            let mut digits: Vec<u32> = vec![];
            for (i, c) in line.char_indices() {
                let remainder_str = &line[i..];
                for (k, v) in &num_dict {
                    if remainder_str.starts_with(k) {
                        digits.push(*v);
                        println!("Found word digit: '{}' -> {}", remainder_str, v);
                    }
                }
                if let Some(digit) = c.to_digit(10) {
                    digits.push(digit);
                }
            }

            let line_cal_value = digits.first().unwrap() * 10 + digits.last().unwrap();
            total += line_cal_value;
        }

        println!("Total = {}", total);


    }

}
