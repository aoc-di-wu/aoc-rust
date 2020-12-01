use std::time::Instant;

mod part1;
mod part2;

fn main() {
    // 1. Get the input.
    let input_raw = include_str!("./input");

    // 2. Split into numbers.
    let numbers: Vec<usize> = input_raw
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let start = Instant::now();
    // part1::naive(&numbers);
    println!("Solution for PART 1: {}", part1::better(&numbers));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    // part2::naive(&numbers);
    println!("Solution for PART 2: {}", part2::better(&numbers));
    println!("Finished after {:?}", start.elapsed());
}
