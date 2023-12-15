use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();

    let sections: Vec<Section> = input
        .split("\r\n\r\n")
        .map(|section| section.lines().map(|l| l.bytes().collect()).collect())
        .map(|v| Section::new(v))
        .collect();

    let mut total_vert = 0;
    let mut total_horz = 0;
    for (i, section) in sections.iter().enumerate() {
        println!("Section #{i}: ");
        if let Some(vert) = section.scan_vertical() {
            println!("\tV = {vert}");
            total_vert += vert;
        }
        if let Some(horz) = section.scan_horizontal() {
            println!("\tH = {horz}");
            total_horz += horz;
        }
    }

    println!(
        "Total H = {total_horz}, Total V = {total_vert} ==> SCORE = {}",
        total_horz * 100 + total_vert
    );
}

struct Section {
    /// Outer Vec is rows, inner Vec is columns
    tiles: Vec<Vec<u8>>,
}

impl Section {
    pub fn new(tiles: Vec<Vec<u8>>) -> Self {
        Self { tiles }
    }
    /// Scans for horizontal symmetry line, and returns number of lines above it.
    pub fn scan_horizontal(&self) -> Option<usize> {
        for (idx, row) in self.tiles.iter().enumerate() {
            if let Some(next_row) = self.tiles.get(idx + 1) {
                if row == next_row {
                    let suspect_idx = idx + 1;
                    if self.verify_horizontal(suspect_idx) {
                        return Some(suspect_idx);
                    }
                }
            }
        }
        None
    }

    pub fn scan_vertical(&self) -> Option<usize> {
        let width = self.width();

        for col_idx in 0..width {
            if let Some(col_data) = self.column(col_idx) {
                if let Some(next_col_data) = self.column(col_idx + 1) {
                    if col_data == next_col_data {
                        let suspect_idx = col_idx + 1;
                        if self.verify_vertical(suspect_idx) {
                            return Some(suspect_idx);
                        }
                    }
                }
            }
        }

        None
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

    pub fn row(&self, idx: usize) -> Option<Vec<u8>> {
        self.tiles.get(idx).cloned()
    }

    pub fn column(&self, idx: usize) -> Option<Vec<u8>> {
        if idx < self.width() {
            Some(self.tiles.iter().map(|r| r[idx]).collect())
        } else {
            None
        }
    }
}
