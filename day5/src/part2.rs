use std::{fmt::Display, fs::File, io::BufReader, ops::Range, time::Instant};

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let is_part2 = args.contains(&"part2".to_string());
    let is_example = args.contains(&"example".to_string());

    let input_file = if is_example { "./example.txt" } else { "./input_1.txt" };

    let input = std::fs::read_to_string(input_file).unwrap();
    let section_txts: Vec<&str> = input.split("\r\n\r\n").collect();

    let seeds: Vec<u64> = section_txts
        .first()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();


    let seed_ranges: Vec<Range<u64>> = if is_part2 {
        seeds
        .chunks(2)
        .map(|c| {
            let start = c[0];
            let len = c[1];

            start..(start + len)
        })
        .collect()
    } else {
        seeds.iter().map(|&s| s..s+1).collect()
    };

    dbg!(&seed_ranges);
    let section_rest: Vec<&&str> = section_txts.iter().skip(1).collect();

    let sections: Vec<Section> = section_rest.iter().map(|st| Section::new(st)).collect();

    // dbg!(sections);

    let soil = sections.iter().find(|s| s.title.dst == "soil").unwrap();
    let fertilizer = sections
        .iter()
        .find(|s| s.title.dst == "fertilizer")
        .unwrap();
    let water = sections.iter().find(|s| s.title.dst == "water").unwrap();
    let light: &Section = sections.iter().find(|s| s.title.dst == "light").unwrap();
    let temperature = sections
        .iter()
        .find(|s| s.title.dst == "temperature")
        .unwrap();
    let humidity = sections.iter().find(|s| s.title.dst == "humidity").unwrap();
    let location = sections.iter().find(|s| s.title.dst == "location").unwrap();

    let mut locations = vec![];

    for seed_range in seed_ranges {
        println!("Seed range = {} samples", seed_range.end - seed_range.start);
        let start = Instant::now();
        for seed in seed_range {
            let soil = soil.lookup(seed);
            let fert = fertilizer.lookup(soil);
            let water = water.lookup(fert);
            let light = light.lookup(water);
            let temp = temperature.lookup(light);
            let hum = humidity.lookup(temp);
            let loc = location.lookup(hum);

            // println!("Seed {seed} -> Location {loc}");

            locations.push(loc);
        }
        let end = Instant::now();
        println!("{:?}", end - start);
    }

    let closest = locations.iter().min().unwrap();

    println!("Closest location = {closest}");
}

#[derive(Debug, Clone)]
struct Section {
    title: SectionTitle,
    ranges: Vec<SectionRange>,
}

#[derive(Debug, Clone)]
struct SectionTitle {
    src: String,
    dst: String,
}

impl SectionTitle {
    pub fn new(line: &str) -> SectionTitle {
        let parts: Vec<&str> = line.split(&[' ', '-']).collect();
        SectionTitle {
            src: parts[0].to_owned(),
            dst: parts[2].to_owned(),
        }
    }
}

impl Display for SectionTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-to-{} map", self.src, self.dst)
    }
}

impl Section {
    pub fn new(section_txt: &str) -> Self {
        let lines: Vec<&str> = section_txt.split("\r\n").collect();
        let title = SectionTitle::new(lines.first().unwrap());

        let mut ranges = vec![];

        for &line in lines.iter().skip(1) {
            if !line.is_empty() {
                let range = SectionRange::new(line);
                ranges.push(range);
            }
        }

        Section { title, ranges }
    }

    pub fn lookup(&self, input: u64) -> u64 {
        for range in &self.ranges {
            if range.src_range.contains(&input) {
                let offset = input - range.source;
                let output = range.destination + offset;
                // println!("{}: {} -> {} -> {}", self.title, input, range, output);
                return output as u64;
            }
        }
        // println!("{}: not found: {} -> {}", self.title, input, input);
        input
    }
}

#[derive(Debug, Clone)]
struct SectionRange {
    source: u64,
    destination: u64,
    margin: u64,
    src_range: Range<u64>,
    dst_range: Range<u64>,
}

impl SectionRange {
    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(" ").collect();
        assert!(parts.len() == 3, "parts={:?}", parts);

        let destination = parts[0].parse::<u64>().expect("Invalid destination number");
        let source = parts[1].parse::<u64>().expect("Invalid source number");
        let margin = parts[2].parse::<u64>().expect("Invalid margin number");

        Self {
            source,
            destination,
            margin,
            src_range: source..source + margin,
            dst_range: destination..destination + margin,
        }
    }
}

impl Display for SectionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} / {:?})", self.src_range, self.dst_range)
    }
}
