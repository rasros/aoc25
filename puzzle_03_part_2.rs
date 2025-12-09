use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut total: u128 = 0;
    let target_len: usize = 12;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).expect("expected digit") as u8)
            .collect();

        let mut to_drop: usize = digits.len() - target_len;
        let mut stack: Vec<u8> = Vec::new();

        for d in digits {
            while to_drop > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
                stack.pop();
                to_drop -= 1;
            }
            stack.push(d);
        }

        stack.truncate(target_len);

        let mut val_str = String::with_capacity(target_len);
        for d in stack {
            val_str.push(char::from(b'0' + d));
        }

        let bank_val: u128 = val_str.parse().expect("failed to parse bank value");
        total += bank_val;
    }

    println!("{}", total);
}
