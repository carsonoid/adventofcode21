use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("filename arg is required");
        return;
    }

    let filename = &args[1];

    println!("Reading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("error reading file");

    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in contents.trim().split("\n") {
        let mut row = Vec::new();
        for c in line.chars() {
            println!("{}", c);
            row.push(c as u32 - '0' as u32);
        }
        grid.push(row);
    }

    for row in &grid {
        println!("{:?}", row);
    }

    let max_width = grid[0].len();
    let max_height = grid.len();

    let mut total_risk = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            // left
            if x > 0 && row[x - 1] <= *height {
                continue;
            }
            // right
            if x < max_width - 1 && row[x + 1] <= *height {
                continue;
            }
            // up
            if y > 0 && grid[y - 1][x] <= *height {
                continue;
            }
            // down
            if y < max_height - 1 && grid[y + 1][x] <= *height {
                continue;
            }
            println!("{},{} is low point of {}", x, y, height);
            total_risk += height + 1;
        }
    }

    println!("total risk: {}", total_risk)
}

// pub fn get_depth(row: &Vec<Vec<u32>>, x: u32, y: u32) -> u32 {}
