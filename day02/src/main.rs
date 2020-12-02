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
    input
        .lines()
        .filter(|l| {
            let (min, max, character, value) = split(l);

            // Count the amount of values that match the character.
            let count = value
                .chars()
                .filter(|c| *c == character)
                .count();

            // The amount must be in the range [min, max].
            return min <= count && count <= max;
        })
        .count()
}

fn part2(input: &str) -> usize {
    input.lines()
        .filter(|l| {
            let (fst, snd, character, value) = split(l);
            // Split value into its characters.
            let characters: Vec<char> = value
                .chars()
                .collect();

            // If the first character matches as the character, check if the
            // second character does not match. Otherwise, the second character
            // must be the same.
            if characters[fst - 1] == character {
                return characters[snd - 1] != character;
            }
            return characters[snd - 1] == character;
        })
        .count()
}

fn split(line: &str) -> (usize, usize, char, &str) {
    // Format of the given line: {min}-{max} {character}: {value}
    let parts: Vec<&str> = line
        .split_whitespace()
        .collect();

    let range: Vec<usize> = parts[0]
        .split('-')
        .map(|c| c.parse().unwrap())
        .collect();
    let min = range[0];
    let max = range[1];

    let character = parts[1]
        .chars()
        .nth(0)
        .unwrap();

    let value = parts[2];

    return (min, max, character, value);
}
