use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut raw_lines: Vec<&str> = Vec::new();
    for line in input.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        raw_lines.push(line);
    }

    if raw_lines.is_empty() {
        println!("0");
        return;
    }

    let mut max_len = 0usize;
    for l in &raw_lines {
        if l.len() > max_len {
            max_len = l.len();
        }
    }

    let mut grid: Vec<Vec<char>> = Vec::new();
    for l in raw_lines {
        let mut row: Vec<char> = l.chars().collect();
        if row.len() < max_len {
            row.resize(max_len, ' ');
        }
        grid.push(row);
    }

    let rows = grid.len();
    let cols = max_len;

    if rows < 2 || cols == 0 {
        println!("0");
        return;
    }

    let mut col_all_space: Vec<bool> = vec![true; cols];
    for c in 0..cols {
        for r in 0..rows {
            if grid[r][c] != ' ' {
                col_all_space[c] = false;
                break;
            }
        }
    }

    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut in_block = false;
    let mut start = 0usize;

    for c in 0..cols {
        if col_all_space[c] {
            if in_block {
                blocks.push((start, c - 1));
                in_block = false;
            }
        } else {
            if !in_block {
                start = c;
                in_block = true;
            }
        }
    }
    if in_block {
        blocks.push((start, cols - 1));
    }

    let last_row = rows - 1;
    let mut grand_total: i64 = 0;

    for (cs, ce) in blocks {
        let mut op: Option<char> = None;
        for c in cs..=ce {
            let ch = grid[last_row][c];
            if ch != ' ' {
                op = Some(ch);
                break;
            }
        }
        let op = match op {
            Some(ch) => ch,
            None => continue,
        };

        let mut numbers: Vec<i64> = Vec::new();

        for c in (cs..=ce).rev() {
            let mut s = String::new();
            for r in 0..last_row {
                let ch = grid[r][c];
                if ch.is_ascii_digit() {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                if let Ok(v) = s.parse::<i64>() {
                    numbers.push(v);
                }
            }
        }

        if numbers.is_empty() {
            continue;
        }

        let mut result: i64;
        if op == '+' {
            result = 0;
            for v in numbers {
                result += v;
            }
        } else if op == '*' {
            result = 1;
            for v in numbers {
                result *= v;
            }
        } else {
            continue;
        }

        grand_total += result;
    }

    println!("{}", grand_total);
}
