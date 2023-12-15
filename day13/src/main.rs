use std::collections::HashMap;

use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();

    let mut sections: Vec<Section> = input
        .split("\r\n\r\n")
        .map(|section| {
            section
                .lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        })
        .map(|v| Section::new(v))
        .collect();

    let mut total_vert = 0;
    let mut total_horz = 0;

    let mut pre_valid_results = HashMap::new();

    for (i, section) in sections.iter().enumerate() {
        println!("Section #{i}: ");
        for sus in section.scan_vertical() {
            if section.verify_vertical(sus) {
                println!("\tV = {sus}");
                pre_valid_results.insert(
                    i,
                    PreValidResult {
                        mirror_idx: sus,
                        is_vertical: true,
                    },
                );
                total_vert += sus;
            }
        }
        for sus in section.scan_horizontal() {
            if section.verify_horizontal(sus) {
                pre_valid_results.insert(
                    i,
                    PreValidResult {
                        mirror_idx: sus,
                        is_vertical: false,
                    },
                );
                println!("\tH = {sus}");
                total_horz += sus;
            }
        }
    }

    total_vert = 0;
    total_horz = 0;

    'sections: for (i, section) in sections.iter_mut().enumerate() {
        println!("Section #{i}: ");
        let pre_result = pre_valid_results.get(&i).unwrap();
        for row in 0..section.height() {
            for col in 0..section.width() {
                if i == 1 && row == 6 && col == 10 {
                    println!("DEBUG");
                }
                let mut bit = section.tiles[row][col];
                bit = !bit;
                section.tiles[row][col] = bit;

                for sus in section.scan_vertical() {
                    if section.verify_vertical(sus) {
                        if pre_result.mirror_idx != sus || !pre_result.is_vertical {
                            println!("\tV = {sus}");
                            total_vert += sus;
                            continue 'sections;
                        }
                    }
                }
                for sus in section.scan_horizontal() {
                    if section.verify_horizontal(sus) {
                        if pre_result.mirror_idx != sus || pre_result.is_vertical {
                            println!("\tH = {sus}");
                            total_horz += sus;
                            continue 'sections;
                        }
                    }
                }

                section.tiles[row][col] = !bit;
            }
        }
    }

    println!(
        "Total H = {total_horz}, Total V = {total_vert} ==> SCORE = {}",
        total_horz * 100 + total_vert
    );
}

struct PreValidResult {
    mirror_idx: usize,
    is_vertical: bool,
}

struct Section {
    /// Outer Vec is rows, inner Vec is columns
    tiles: Vec<Vec<bool>>,
}

impl Section {
    pub fn new(tiles: Vec<Vec<bool>>) -> Self {
        Self { tiles }
    }
    /// Scans for horizontal symmetry line, and returns number of lines above it.
    pub fn scan_horizontal(&self) -> Vec<usize> {
        let mut suspects = vec![];
        for (idx, row) in self.tiles.iter().enumerate() {
            if let Some(next_row) = self.tiles.get(idx + 1) {
                if row == next_row {
                    let suspect_idx = idx + 1;
                    // if self.verify_horizontal(suspect_idx) {
                    //     return Some(suspect_idx);
                    // }
                    suspects.push(suspect_idx);
                }
            }
        }
        suspects
    }

    pub fn scan_vertical(&self) -> Vec<usize> {
        let width = self.width();
        let mut suspects = vec![];

        for col_idx in 0..width {
            if let Some(col_data) = self.column(col_idx) {
                if let Some(next_col_data) = self.column(col_idx + 1) {
                    if col_data == next_col_data {
                        let suspect_idx = col_idx + 1;
                        // if self.verify_vertical(suspect_idx) {
                        //     return Some(suspect_idx);
                        // }
                        suspects.push(suspect_idx);
                    }
                }
            }
        }

        suspects
    }

    /// Walks the columns left and right of suspected mirror to verify it's actually mirrored.
    pub fn verify_horizontal(&self, row: usize) -> bool {
        let mut bottom_idx = row;
        let mut top_idx = row - 1;
        loop {
            if let Some(top) = self.row(bottom_idx) {
                if let Some(bottom) = self.row(top_idx) {
                    if top != bottom {
                        return false;
                    } else {
                        if top_idx == 0 {
                            break;
                        }
                        bottom_idx += 1;
                        top_idx -= 1;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        true
    }

    /// Walks the columns left and right of suspected mirror to verify it's actually mirrored.
    pub fn verify_vertical(&self, col: usize) -> bool {
        let mut right_idx = col;
        let mut left_idx = col - 1;
        loop {
            if let Some(right) = self.column(right_idx) {
                if let Some(left) = self.column(left_idx) {
                    if right != left {
                        return false;
                    } else {
                        if left_idx == 0 {
                            break;
                        }
                        right_idx += 1;
                        left_idx -= 1;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        true
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn row(&self, idx: usize) -> Option<Vec<bool>> {
        self.tiles.get(idx).cloned()
    }

    pub fn column(&self, idx: usize) -> Option<Vec<bool>> {
        if idx < self.width() {
            Some(self.tiles.iter().map(|r| r[idx]).collect())
        } else {
            None
        }
    }
}
