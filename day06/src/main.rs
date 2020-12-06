use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let raw_input = include_str!("./input");

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&raw_input));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&raw_input));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(input: &str) -> usize {
    return input
        .split("\n\n")
        .map(|group| group
            .split_whitespace()
            .flat_map(|person| person.chars())
            .collect::<HashSet<char>>()
            .len()
        )
        .sum();
}

fn part2(input: &str) -> usize {
    return input
        .split("\n\n")
        .map(|group| {
            let people = group
                .split_whitespace()
                .map(|person| person.chars().collect())
                .collect::<Vec<HashSet<char>>>();

            ('a'..='z')
                .filter(|c|
                    people
                        .iter()
                        .all(|s| s.contains(c)))
                .count()
        })
        .sum();
}
