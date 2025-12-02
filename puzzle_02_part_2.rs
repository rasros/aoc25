use std::io::{self, Read};

fn generate_invalid_ids(max_val: u64) -> Vec<u64> {
    let mut invalids = Vec::new();
    if max_val < 11 {
        return invalids;
    }

    let max_digits = max_val.to_string().len() as u32;

    for k in 1..=(max_digits / 2) {
        let pow10k = 10_u64.pow(k);
        let start = pow10k / 10;
        let end = pow10k - 1;

        let mut base = start;
        while base <= end {
            let mut val = base * pow10k + base;
            if val > max_val {
                base += 1;
                continue;
            }
            invalids.push(val);

            loop {
                let mul = match val.checked_mul(pow10k) {
                    Some(v) => v,
                    None => break,
                };
                let next = match mul.checked_add(base) {
                    Some(v) => v,
                    None => break,
                };

                if next > max_val {
                    break;
                }

                invalids.push(next);
                val = next;
            }

            base += 1;
        }
    }

    invalids.sort_unstable();
    invalids.dedup();
    invalids
}

fn sum_invalids_in_ranges(invalids: &[u64], ranges: &[(u64, u64)]) -> u128 {
    let mut ranges_sorted = ranges.to_vec();
    ranges_sorted.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (s, e) in ranges_sorted {
        if merged.is_empty() {
            merged.push((s, e));
        } else {
            let last = merged.last_mut().unwrap();
            if s > last.1.saturating_add(1) {
                merged.push((s, e));
            } else if e > last.1 {
                last.1 = e;
            }
        }
    }

    let mut total: u128 = 0;
    let mut idx = 0usize;

    for (l, r) in merged {
        while idx < invalids.len() && invalids[idx] < l {
            idx += 1;
        }
        while idx < invalids.len() && invalids[idx] <= r {
            total += invalids[idx] as u128;
            idx += 1;
        }
    }

    total
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let line = input.trim();
    if line.is_empty() {
        println!("0");
        return;
    }

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for part in line.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let mut it = part.split('-');
        let start_str = it.next().expect("missing start");
        let end_str = it.next().expect("missing end");

        let start: u64 = start_str.parse().expect("invalid start number");
        let end: u64 = end_str.parse().expect("invalid end number");

        if start > end {
            panic!("range start > end: {}-{}", start, end);
        }

        ranges.push((start, end));
    }

    if ranges.is_empty() {
        println!("0");
        return;
    }

    let mut max_val = ranges[0].1;
    for &(_, e) in &ranges {
        if e > max_val {
            max_val = e;
        }
    }

    let invalids = generate_invalid_ids(max_val);
    let total = sum_invalids_in_ranges(&invalids, &ranges);

    println!("{}", total);
}

