use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin().lock();
    let mut sum: i32 = 0;
    for line in stdin.lines() {
        let val = line.unwrap();
        let mut num_str = String::with_capacity(2);
        for c in val.chars() {
            if c.is_digit(10) {
                num_str.push(c);
                break;
            }
        }
        for c in val.chars().rev() {
            if c.is_digit(10) {
                num_str.push(c);
                break;
            }
        }
        sum += num_str.parse::<i32>().unwrap();
    }
    println!("{}", sum);
}
