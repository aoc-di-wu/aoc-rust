mod part1;
mod part2;

fn main() {
    // 1. Get the input.
    let input_raw = include_str!("./input");

    // 2. Split into numbers.
    let numbers: Vec<i32> = input_raw
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    // part1::naive(&numbers);
    println!("Solution for PART 1: {}", part1::better(&numbers));

    // part2::naive(&numbers);
    println!("Solution for PART 2: {}", part2::better(&numbers));
}
