use std::time::Instant;

fn main() {
    let raw_input = include_str!("./input");
    let grid: Vec<Vec<bool>> = raw_input
        .lines()
        .map(|x| x
            .chars()
            .map(|x| x == '#') // Map characters to booleans.
            .collect()
        )
        .collect();

    let start = Instant::now();
    // part1::naive(&numbers);
    println!("Solution for PART 1: {}", part1(&grid));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    // part1::naive(&numbers);
    println!("Solution for PART 2: {}", part2(&grid));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(grid: &Vec<Vec<bool>>) -> usize {
    return traverse(grid, 3, 1);
}

fn part2(grid: &Vec<Vec<bool>>) -> usize {
    return traverse(grid, 1, 1)
        * traverse(grid, 3, 1)
        * traverse(grid, 5, 1)
        * traverse(grid, 7, 1)
        * traverse(grid, 1, 2);
}

fn traverse(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut trees: usize = 0; // Amount of trees encountered.
    let mut x_pos: usize = 0; // Current x position.
    for row in (0..grid.len()).step_by(y) {
        // Add one to trees if we encounter one.
        if grid[row][x_pos] {
            trees += 1;
        }

        // Move x to the right.
        x_pos += x;
        // Wrap around of x_pos >= length of the row.
        x_pos %= grid[row].len();
    }
    return trees;
}