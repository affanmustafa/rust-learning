/*
   Given a list of integers, use a vector and return the median (when sorted, the value in the
   middle position) and mode (the value that occurs the most often; a hashmap will be helpful here)
   of the list.
*/
use std::collections::HashMap;

fn main() {
    let mut integers: Vec<i32> = vec![1, 2, 3, 5, 4, 10, 20, 12, 53, 77, 14];
    integers.sort();
    println!("Sorted vector {:?}", integers);

    let median = if integers.len() % 2 == 0 {
        let mid1 = integers.len() / 2 - 1;
        let mid2 = integers.len() / 2;
        (integers[mid1] + integers[mid2]) / 2
    } else {
        integers[integers.len() / 2]
    };
    println!("Median: {}", median);

    let mut integers_map = HashMap::new();
    for &integer in &integers {
        *integers_map.entry(integer).or_insert(0) += 1;
    }

    let mode = integers_map.into_iter().max_by_key(|&(_, count)| count).map(|(integer, _)| integer).unwrap();
    println!("Mode: {}", mode);
    // code for looping through vector
}

// use std::collections::HashMap;

// fn calculate_median_mode(numbers: Vec<i32>) -> (f32, i32) {
//     let mut numbers = numbers.clone(); // Clone to avoid mutating the original vector
//     numbers.sort(); // Sort the vector to find the median

//     // Calculate the median
//     let median = if numbers.len() % 2 == 0 {
//         let mid1 = numbers.len() / 2 - 1;
//         let mid2 = numbers.len() / 2;
//         (numbers[mid1] + numbers[mid2]) as f32 / 2.0
//     } else {
//         numbers[numbers.len() / 2] as f32
//     };

//     // Calculate the mode
//     let mut occurrences = HashMap::new();
//     for &number in &numbers {
//         *occurrences.entry(number).or_insert(0) += 1;
//     }

//     let mode = occurrences.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| num).unwrap();

//     (median, mode)
// }

// fn main() {
//     let numbers = vec![1, 2, 2, 3, 4, 4, 4, 5, 5];
//     let (median, mode) = calculate_median_mode(numbers);

//     println!("Median: {}", median);
//     println!("Mode: {}", mode);
// }
