use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        grid.push(line.chars().collect());
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut total_removed: i64 = 0;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' {
                    continue;
                }

                let mut neighbor_rolls = 0;

                for (dr, dc) in directions.iter() {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr < 0 || nc < 0 {
                        continue;
                    }
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if nr >= rows || nc >= cols {
                        continue;
                    }

                    if grid[nr][nc] == '@' {
                        neighbor_rolls += 1;
                    }
                }

                if neighbor_rolls < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove {
            grid[r][c] = '.';
            total_removed += 1;
        }
    }

    println!("{}", total_removed);
}


