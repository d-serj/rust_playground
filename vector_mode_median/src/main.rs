
use std::collections::HashMap;

fn main() {
    let mut vec = vec![3, 4, 52, 5, 10, 12, 10, 95, 32, 15, 11, 3, 10, 43, 12, 14, 11];
    vec.sort();
    println!("vec {:?}", vec);
    println!("median of vector is: {}", get_median_of_vec(&vec));
    println!("mode of vector is {}", get_mode_of_vec(&vec));
}

fn get_median_of_vec(vec: &Vec<i32>) -> i32 {
    *vec.get(vec.len() / 2).unwrap()
}

fn get_mode_of_vec(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut prev_count = 0;
    let mut mode = 0;
    
    for i in vec {
        let count = map.entry(i).or_insert(0);

        *count += 1;
        
        if prev_count < *count {
            prev_count = *count;
            mode = *i;
        }
    }

    mode
}
