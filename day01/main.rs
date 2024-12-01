use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]
fn sort(vec: &mut Vec<i32>) {
    let n = vec.len();

    for i in 0..n {
        for j in 0..n-i-1 {
            if vec[j] > vec[j+1] {
                vec.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut map2: HashMap<i32, i32> = HashMap::new();
    let mut map1: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<Result<i32, _>> = line.split("   ").map(|no| no.parse()).collect();

        if numbers.len() >= 2 {
            if let (Ok(num1), Ok(num2)) = (&numbers[0], &numbers[1]) {
                let no1: i32 = *num1;
                let no2: i32 = *num2;

                if map2.contains_key(&no2) {
                    if let Some(value) = map2.get_mut(&no2) {
                        *value += 1;
                    }
                } else {
                    map2.insert(no2, 1i32);
                }

                if map1.contains_key(&no1) {
                    if let Some(value) = map1.get_mut(&no1) {
                        *value += 1;
                    }
                } else {
                    map1.insert(no1, 1i32); // Corrected this line
                }

            } else {
                eprintln!("Failed to parse numbers on line: {}", line);
            }
        } else {
            eprintln!("Not enough numbers on line: {}", line);
        }
    }

    // sort(&mut vec1);
    // sort(&mut vec2);

    let mut sum: i64 = 0;

    for (contact, &number) in map1.iter() {
        if map2.contains_key(&contact){
            let occurance = map2.get(&contact);
            sum += (number * contact * occurance.unwrap()) as i64;
        }
    }
    println!("Sum of absolute differences: {}", sum);
}