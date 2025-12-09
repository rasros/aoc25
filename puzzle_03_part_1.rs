use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut total: i64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).expect("expected digit"))
            .collect();

        if digits.len() < 2 {
            continue;
        }

        let mut best: i32 = 0;

        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                let val = (digits[i] * 10 + digits[j]) as i32;
                if val > best {
                    best = val;
                }
            }
        }

        total += best as i64;
    }

    println!("{}", total);
}

