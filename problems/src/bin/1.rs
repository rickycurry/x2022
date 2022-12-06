use parser;
use std::cmp;

fn main() {
    let input = parser::parse("1");
    let mut totals: Vec<u32> = vec![0];
    for s in input {
        let val = s.parse::<u32>();
        if val.is_ok() {
            *totals.last_mut().unwrap() += val.unwrap();
        }
        else {
            totals.push(0);
        }
    }

    let mut max: u32 = 0;
    for t in &totals {
        max = cmp::max(max, *t);
    }

    println!("1. max total is {}", max);

    totals.sort_unstable();
    totals.reverse();
    println!("2. total of top 3 is {}", totals[0] + totals[1] + totals[2]);
}