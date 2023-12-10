use ranges::{GenericRange as Range, Ranges};

fn main() {
    let input = dbg!(Ranges::from(0..20));

    let diff = dbg!(Ranges::from(vec![2..3, 6..7]));

    let sub = dbg!(input.difference(diff));


}