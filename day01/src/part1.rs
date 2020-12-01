use std::collections::HashSet;

const TARGET: i32 = 2020;

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
pub fn naive(numbers: &Vec<i32>) -> i32 {
    for e1 in numbers {
        for e2 in numbers {
            if e1 + e2 == TARGET {
               return e1 * e2;
            }
        }
    }
    return -1;
}

// PART 1 | BETTER SOLUTION
// Option 2: O(n)
// Use a HashSet to keep track of the complements.
pub fn better(numbers: &Vec<i32>) -> i32 {
    let mut complements: HashSet<i32> = HashSet::new();
    for e in numbers {
        let complement = TARGET - e;
        if complements.contains(&complement) {
            return e * complement;
        }
        complements.insert(*e);
    }
    return -1;
}
