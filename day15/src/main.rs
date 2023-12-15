use tools::Opt;

fn main() {
    let opt = Opt::load();
    let input = opt.input();
    
    let seq_items: Vec<&str> = input.split(&[',','\r', '\n']).filter(|s| !s.is_empty()).collect();
    let hash_sum = seq_items.iter().fold(0usize, |acc, seq| acc + hash(seq) as usize);
    println!("Hash Sum = {hash_sum}");

}

fn hash(input: &str) -> u8 {
    let bytes = input.as_bytes();
    let mut acc = 0usize;
    for b in bytes {
        acc += *b as usize;
        acc *= 17;
        acc %= 256;
    }
    acc as u8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn hash_test(){
        let input = "HASH";
        let res = hash(input);

        assert_eq!(res, 52u8);
    }
}