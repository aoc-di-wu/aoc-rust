use std::collections::HashMap;

const TARGET: i32 = 2020;

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
pub fn naive(numbers: &Vec<i32>) -> i32 {
    for e1 in numbers {
        for e2 in numbers {
            for e3 in numbers {
                if e1 + e2 + e3 == 2020 {
                    return e1 * e2 * e3;
                }
            }
        }
    }
    return -1;
}

// PART 2 | BETTER SOLUTION
// Option 2: O(n^2)
// Use a HashMap to keep track of the complements.
pub fn better(numbers: &Vec<i32>) -> i32 {
    for e1 in numbers {
        let new_target = TARGET - e1;
        let mut complements: HashMap<i32,i32> = HashMap::new();
        for e2 in numbers {
            let complement = new_target - e2;
            if complements.contains_key(&complement) {
                return complement * e1 * e2;
            }
            complements.insert(*e2, complement);
        }
    }
    return -1;
}
