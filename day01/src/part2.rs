use std::collections::HashSet;

const TARGET: usize = 2020;

// PART 2 | NAIVE SOLUTION
// Option 1: O(n^3)
// for e1 in n {
//     for e2 in n {
//         for e3 in n {
//             // e1 != e2 != e3
//             if e1 + e2 + e3 == 2020
//             return 
//         }
//     }
// }
#[allow(dead_code)]
pub fn naive(numbers: &Vec<usize>) -> usize {
    for i in 0..numbers.len() {
        let e1 = numbers[i];
        for j in i..numbers.len() {
            let e2 = numbers[j];
            for k in j..numbers.len() {
                let e3 = numbers[k];
                if e1 + e2 + e3 == 2020 {
                    return e1 * e2 * e3;
                }
            }
        }
    }
    return 0;
}

// PART 2 | BETTER SOLUTION
// Option 2: O(n^2)
// Use a HashSet to keep track of the complements.
pub fn better(numbers: &Vec<usize>) -> usize {
    for i in 0..numbers.len() {
        let e1 = numbers[i];
        let new_target = TARGET - e1;
        let mut complements: HashSet<usize> = HashSet::new();
        for j in i..numbers.len() {
            let e2 = numbers[j];
            let complement = new_target - e2;
            if complements.contains(&complement) {
                return complement * e1 * e2;
            }
            complements.insert(e2);
        }
    }
    return 0;
}
