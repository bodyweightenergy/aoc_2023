use ranges::{GenericRange, OperationResult, Ranges};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::BufReader,
    ops::{Bound, RangeBounds},
    time::Instant,
};

use ranges::GenericRange as Range;
type Rg = Range<u64>;
type Rgs = Ranges<u64>;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let is_part2 = args.contains(&"part2".to_string());
    let is_example = args.contains(&"example".to_string());

    let input_file = if is_example {
        "./example.txt"
    } else {
        "./input_1.txt"
    };

    let input = std::fs::read_to_string(input_file).unwrap();
    let section_txts: Vec<&str> = input.split("\r\n\r\n").collect();

    let seeds: Vec<u64> = section_txts
        .first()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let seed_ranges: Vec<Rg> = if is_part2 {
        seeds
            .chunks(2)
            .map(|c| {
                let start = c[0];
                let len = c[1];

                (start..(start + len)).into()
            })
            .collect()
    } else {
        seeds.iter().map(|&s| (s..s + 1).into()).collect()
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
        let start = Instant::now();
        let soil = soil.lookup_ranges(seed_range);
        let fert = fertilizer.lookup_ranges(soil);
        let water = water.lookup_ranges(fert);
        let light = light.lookup_ranges(water);
        let temp = temperature.lookup_ranges(light);
        let hum = humidity.lookup_ranges(temp);
        let loc = location.lookup_ranges(hum);

        // println!("Seed {seed} -> Location {loc}");

        locations.push(loc);
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

    /// Finds a single seed in this section
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

    /// Finds all mapped ranges for the provided seed range.
    pub fn lookup_ranges(&self, input: Rgs) -> Rgs {
        let mut mapped_pairs: Vec<(Rgs, Rgs)> = vec![];

        // Find where input intersects with map
        for range in &self.ranges {
            // match input.intersect(range.src_range) {
            //     _ => {}
            //     OperationResult::Single(intersect) => {
            //         let intersect_out = range.transform(intersect);
            //         mapped_pairs.push((intersect, intersect_out));
            //     }
            // }
            let src_ranges = Rgs::from(range.src_range);
            let src_intersect = src_ranges.intersect(input);
            let transformed_intersect = src_intersect.as_slice().iter().map(|r| )
        }

        let mapped_sources = Rgs::from(
            mapped_pairs
                .iter()
                .map(|(p, _)| p.clone())
                .collect::<Vec<Rg>>(),
        );

        let input_ranges = Rgs::from(input);
        let unmapped = input_ranges.difference(mapped_sources);

        let mapped_dest = Rgs::from(
            mapped_pairs
                .iter()
                .map(|(_, t)| t.clone())
                .collect::<Vec<Rg>>(),
        );
        mapped_dest.union(unmapped)
    }
}

struct SectionPath {
    input: Rg,
    in_range: HashMap<SectionRange, Option<Rg>>,
    unmapped: Rgs,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct SectionRange {
    source: u64,
    destination: u64,
    margin: u64,
    src_range: Rg,
    dst_range: Rg,
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
            src_range: (source..source + margin).into(),
            dst_range: (destination..destination + margin).into(),
        }
    }

    /// Gets the offset of the destination relative to source.
    pub fn offset(&self) -> i64 {
        self.destination as i64 - self.source as i64
    }

    /// Shifts the input range by as much needed, assumes input already in range of source.
    pub fn transform(&self, input: Rgs) -> Rgs {
        let mut outputs = vec![];
        for r in input.as_slice() {
        if let Bound::Included(start) = input.start_bound() {
            if let Bound::Excluded(end) = input.end_bound() {
                let offset = self.offset();
                let new_start = *start as i64 + offset;
                let new_end = *end as i64 + offset;
                return (new_start as u64..new_end as u64).into();
            }
        }
    }
        panic!("Not supposed to have ranges with different bound kinds.");
    }
}

impl Display for SectionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} / {:?})", self.src_range, self.dst_range)
    }
}
