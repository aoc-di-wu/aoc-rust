use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let raw_input = include_str!("./input");

    let instructions: Vec<(&str, isize)> = raw_input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l
                .split(' ')
                .collect();
            let operation = parts[0];
            let argument = parts[1]
                .parse()
                .unwrap();
            return (operation, argument);
        })
        .collect();

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(instructions.clone()));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(instructions.clone()));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(instructions: Vec<(&str, isize)>) -> isize {
    return match run(&instructions) {
        Ok(acc) => acc,
        Err(acc) => acc,
    };
}

fn part2(mut instructions: Vec<(&str, isize)>) -> isize {
    let mut part2 = 0;
    for i in 0..instructions.len() {
        swap(&mut instructions[i]);
        if let Ok(acc) = run(&instructions) {
            part2 = acc;
            break;
        }
        swap(&mut instructions[i]);
    }
    return part2;
}

fn run(instructions: &[(&str, isize)]) -> Result<isize, isize> {
    let mut acc = 0;
    let mut current = 0;
    let mut visited = HashSet::new();
    while current < instructions.len() {
        if visited.contains(&current) {
            return Err(acc);
        }
        visited.insert(current.clone());
        match instructions[current.clone()] {
            ("nop", _) => current += 1,
            ("acc", arg) => {
                acc += arg;
                current += 1;
            }
            ("jmp", arg) => current = (current as isize + arg) as usize,
            (op, _) => panic!("Unknown operation: {}", op),
        }
    }
    return Ok(acc);
}

fn swap(instruction: &mut (&str, isize)) {
    match instruction {
        ("nop", _) => {
            instruction.0 = "jmp";
        }
        ("jmp", _) => {
            instruction.0 = "nop";
        }
        _ => (),
    }
}