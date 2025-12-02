use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut position: i64 = 50;
    let mut count_zero: i64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (dir, rest) = line.split_at(1);
        let steps: i64 = rest.parse().expect("need number");

        let residue = match dir {
            "R" => (100 - position) % 100,
            "L" => position % 100,
            _ => panic!("Invalid direction: {}", dir),
        };

        let first_k = if residue == 0 { 100 } else { residue };

        if steps >= first_k {
            count_zero += 1 + (steps - first_k) / 100;
        }

        match dir {
            "L" => {
                position = (position - steps).rem_euclid(100);
            }
            "R" => {
                position = (position + steps).rem_euclid(100);
            }
            _ => unreachable!(),
        }
    }

    println!("{}", count_zero);
}

