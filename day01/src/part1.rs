use std::collections::HashSet;

const TARGET: usize = 2020;

// PART 1 | NAIVE SOLUTION
// Option 1: O(n^2)
// for e1 in n {
//     for e2 in n {
//         // e1 != e2
//         if e1 + e2 == 2020
//         return 
//     }
// }
#[allow(dead_code)]
pub fn naive(numbers: &Vec<usize>) -> usize {
    for i in 0..numbers.len() {
        let e1 = numbers[i];
        for j in i..numbers.len() {
            let e2 = numbers[j];
            if e1 + e2 == TARGET {
               return e1 * e2;
            }
        }
    }
    return 0;
}

// PART 1 | BETTER SOLUTION
// Option 2: O(n)
// Use a HashSet to keep track of the complements.
pub fn better(numbers: &Vec<usize>) -> usize {
    let mut complements: HashSet<usize> = HashSet::new();
    for i in 0..numbers.len() {
        let e = numbers[i];
        let complement = TARGET - e;
        if complements.contains(&complement) {
            return numbers[e] * complement;
        }
        complements.insert(e);
    }
    return 0;
}
