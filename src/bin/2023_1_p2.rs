use std::io::{self, BufRead};

const NUM_ARRAY: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let stdin = io::stdin().lock();
    let mut sum: usize = 0;
    for line in stdin.lines() {
        let current_str = line.unwrap();
        // forward
        for i in 0..current_str.len() {
            let val = current_str.as_bytes()[i];
            if val.is_ascii_digit() {
                sum += (val as usize - '0' as usize) * 10;
                break;
            }
            let mut found = false;
            for j in 0..NUM_ARRAY.len() {
                let number = NUM_ARRAY[j];
                let l = number.len();
                if i + l > current_str.len() {
                    continue;
                }
                if &current_str[i..(i + l)] == number {
                    sum += (j + 1) * 10;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        for i in (0..current_str.len()).rev() {
            let val = current_str.as_bytes()[i];
            if val.is_ascii_digit() {
                sum += val as usize - '0' as usize;
                break;
            }
            let mut found = false;
            for j in 0..NUM_ARRAY.len() {
                let number = NUM_ARRAY[j];
                let l = number.len();
                if i + l > current_str.len() {
                    continue;
                }
                if &current_str[i..(i + l)] == number {
                    sum += j + 1;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    println!("{}", sum);
}
