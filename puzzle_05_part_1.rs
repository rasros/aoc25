use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ids: Vec<i64> = Vec::new();

    let mut second_section = false;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            second_section = true;
            continue;
        }

        if !second_section {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start: i64 = start_str.parse().unwrap();
            let end: i64 = end_str.parse().unwrap();
            ranges.push((start, end));
        } else {
            let id: i64 = line.parse().unwrap();
            ids.push(id);
        }
    }

    let mut fresh_count: i64 = 0;

    for id in ids {
        let mut fresh = false;

        for (start, end) in ranges.iter() {
            if id >= *start && id <= *end {
                fresh = true;
                break;
            }
        }

        if fresh {
            fresh_count += 1;
        }
    }

    println!("{}", fresh_count);
}

