use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("./input");
    let seat_ids = input
        .lines()
        .map(seat_id);

    let start = Instant::now();
    if let Some(part1) = seat_ids.clone().max()
    {
        println!("Solution for PART 1: {}", part1);
        println!("Finished after {:?}", start.elapsed());
    }

    let start = Instant::now();
    let set: HashSet<_> = seat_ids
        .clone()
        .collect();
    if let Some(part2) = seat_ids
        .clone()
        .find_map(|id| {
            if !set.contains(&(id + 1)) && set.contains(&(id + 2)) {
                return Some(id + 1);
            }
            return None;
        })
    {
        println!("Solution for PART 2: {}", part2);
        println!("Finished after {:?}", start.elapsed());
    }
}

fn seat_id(l: &str) -> usize {
    let (row_part, col_part) = l.split_at(7);
    return find(0, 127, row_part) * 8 + find(0, 7, col_part);
}

fn find(min: usize, max: usize, s: &str) -> usize {
    return s
        .chars()
        .fold(min..=max, |i, c| {
            let half = (i.start() + i.end()) / 2;
            match c {
                'F' | 'L' => *i.start()..=half,
                'B' | 'R' => half..=*i.end(),
                _ => unreachable!(),
            }
        })
        .last()
        .unwrap();
}


#[cfg(test)]
mod test {
    use crate::seat_id;

    #[test]
    fn p1e1() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
    }
}
