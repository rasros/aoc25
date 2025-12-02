use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut position: i32 = 50;
    let mut count_zero = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (dir, rest) = line.split_at(1);
        let steps: i32 = rest.parse().expect("need number");

        match dir {
            "L" => {
                position = (position - steps).rem_euclid(100);
            }
            "R" => {
                position = (position + steps).rem_euclid(100);
            }
            _ => panic!("Invalid direction: {}", dir),
        }

        if position == 0 {
            count_zero += 1;
        }
    }

    println!("{}", count_zero);
}

