use std::time::Instant;

fn main() {
    let raw_input = include_str!("./input");
    let numbers: Vec<usize> = raw_input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();


    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&numbers));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&numbers));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(nums: &Vec<usize>) -> usize {
    for i in 25..nums.len() {
        if !(i - 25..i - 1)
            .any(|j|
                (j + 1..i).any(|k|
                    nums[i] == nums[j] + nums[k]
                )
            )
        {
            return nums[i];
        }
    }
    unreachable!()
}

fn part2(nums: &Vec<usize>) -> usize {
    let (
        target,
        mut sum,
        mut i,
        mut j,
    ) = (part1(nums), 0, 0, 0);

    while i < nums.len() {
        if sum == target && i + 1 < j {
            let num_min = nums[i..j].iter().min();
            let num_max = nums[i..j].iter().max();
            if let Some(min) = num_min {
                if let Some(max) = num_max {
                    return min + max;
                }
            }
        } else if sum < target && j < nums.len() {
            sum += nums[j];
            j += 1;
        } else {
            sum -= nums[i];
            i += 1;
        }
    }
    unreachable!()
}