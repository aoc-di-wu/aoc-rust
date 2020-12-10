use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let raw_input = include_str!("./input");
    let mut numbers: Vec<usize> = raw_input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    numbers.push(0);
    numbers.push(3 + numbers.iter().max().unwrap());
    numbers.sort();

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&numbers));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&numbers));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(numbers: &Vec<usize>) -> usize {
    let mut ones = 0;
    let mut threes = 0;
    for i in 0..(numbers.len() - 1) {
        match numbers[i + 1] - numbers[i] {
            1 => ones += 1,
            3 => threes += 1,
            _ => continue
        }
    }
    return ones * threes;
}

fn part2(numbers: &Vec<usize>) -> usize {
    fn calculate(jolts: usize, numbers: &Vec<usize>, map: &mut HashMap<usize, usize>) -> usize {
        return if let Some(count) = map.get(&jolts) {
            *count
        } else {
            let sum = (1..=3)
                .map(|offset| jolts + offset)
                .filter(|next| numbers
                    .binary_search(&next)
                    .is_ok()
                )
                .map(|next| calculate(next, numbers, map))
                .sum::<usize>()
                .max(1);

            map.insert(jolts.clone(), sum);
            sum
        };
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    return calculate(numbers[0], numbers, &mut map);
}