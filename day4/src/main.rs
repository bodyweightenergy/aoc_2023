use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    let lines: Vec<&str>  = input.lines().collect();

    let mut total_pts = 0;
    for (i, line) in lines.iter().enumerate() {
        let card = Card::new(line);
        let points = card.points();
        println!("Card {i}: {points} pts");
        total_pts += points;
    }
    println!("Total Points = {total_pts}");
    
}

struct Card {
    winning_nums: Vec<usize>,
    my_nums: Vec<usize>,
}

impl Card {
    pub fn new(line: &str) -> Self {
        let vert_parts: Vec<&str> = line.split('|').collect();

        let wins: Vec<usize> = vert_parts[0].split(' ').filter(|s| !s.is_empty()).skip(2).map(|s| s.parse::<usize>().unwrap()).collect();
        let mine: Vec<usize> = vert_parts[1].split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().unwrap()).collect();

        Self {
            winning_nums: wins,
            my_nums: mine,
        }
    }

    pub fn points(&self) -> usize {
        let num = self.my_nums.iter().filter(|n| self.winning_nums.contains(n)).count();
        let pts = if num > 0 { 1 << (num - 1) } else { 0 };
        println!("Num of winning cards = {num}, Points = {pts}");
        pts
    }
}