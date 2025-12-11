use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut ranges: Vec<(i64, i64)> = Vec::new();
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
        }
    }

    if ranges.is_empty() {
        println!("0");
        return;
    }

    ranges.sort_by_key(|r| r.0);

    let mut total_fresh: i64 = 0;
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for i in 1..ranges.len() {
        let (start, end) = ranges[i];
        if start > current_end + 1 {
            total_fresh += current_end - current_start + 1;
            current_start = start;
            current_end = end;
        } else {
            if end > current_end {
                current_end = end;
            }
        }
    }

    total_fresh += current_end - current_start + 1;

    println!("{}", total_fresh);
}

