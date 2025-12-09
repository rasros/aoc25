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

    if grid.is_empty() {
        println!("0");
        return;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut accessible_count: i64 = 0;

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
                accessible_count += 1;
            }
        }
    }

    println!("{}", accessible_count);
}
