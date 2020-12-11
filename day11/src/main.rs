use std::time::Instant;

fn main() {
    let raw_input = include_str!("./input");
    let grid: Vec<Vec<char>> = raw_input
        .lines()
        .map(|l| l
            .chars()
            .collect::<Vec<char>>()
        )
        .collect();

    let start = Instant::now();
    println!("Solution for PART 1: {}", simulate(grid.clone(), part1));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", simulate(grid.clone(), part2));
    println!("Finished after {:?}", start.elapsed());
}

fn simulate<F>(grid: Vec<Vec<char>>, part: F) -> usize
    where
        F: Fn(Vec<Vec<char>>) -> Vec<Vec<char>>,
{
    let mut previous = grid.clone();
    let mut current = part(grid);

    // Loop until nothing changes.
    while previous != current {
        previous = current.clone();
        current = part(current);
    }

    return current
        .iter()
        .flatten()
        .filter(|&&c| c == '#')
        .count();
}

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)
];

fn part1(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = grid.clone();

    let count = |x: i32, y: i32| {
        let mut count = 0;
        for (dx, dy) in DIRECTIONS.iter() {
            if check(&grid, x + dx, y + dy) &&
                get(&grid, x + dx, y + dy) == '#' {
                count += 1
            }
        }
        return count;
    };

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '.' {
                continue;
            }

            let count = count(x as i32, y as i32);
            let occupied = grid[y][x] == '#';
            new_grid[y][x] = if occupied && 4 <= count {
                'L'
            } else if !occupied && count == 0 {
                '#'
            } else {
                grid[y][x]
            }
        }
    }
    return new_grid;
}

fn part2(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = grid.clone();

    let count = |x: i32, y: i32| {
        let mut count = 0;
        for (dx, dy) in DIRECTIONS.iter() {
            let (mut new_x, mut new_y) = (x + dx, y + dy);
            while check(&grid, new_x, new_y) &&
                get(&grid, new_x, new_y) == '.' {
                new_x = new_x + dx;
                new_y = new_y + dy;
            }

            if check(&grid, new_x, new_y) &&
                get(&grid, new_x, new_y) == '#' {
                count += 1;
            }
        }
        return count;
    };

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '.' {
                continue;
            }

            let count = count(x as i32, y as i32);
            let occupied = grid[y][x] == '#';
            new_grid[y][x] = if occupied && 5 <= count {
                'L'
            } else if !occupied && count == 0 {
                '#'
            } else {
                grid[y][x]
            }
        }
    }

    return new_grid;
}

fn get(grid: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    grid[y as usize][x as usize]
}

fn check(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < grid[0].len() as i32 && y < grid.len() as i32
}
