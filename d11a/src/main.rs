use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("filename arg is required");
        return;
    }

    let filename = &args[1];
    let do_steps = &args[2].parse().expect("must be a number");
    println!("Reading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("error reading file");

    let mut grid: Vec<Vec<Point>> = Vec::new();

    for (y, line) in contents.trim().split("\n").enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            // println!("{}", c);
            let e = c as u32 - '0' as u32;
            row.push(Point {
                x: x,
                y: y,
                energy: e,
                flashed: false,
            });
        }
        grid.push(row);
    }

    println!("START");
    print_grid(&grid);

    let mut num_flashes = 0;

    for step in 1..do_steps + 1 {
        println!("\n---Step {}", step);

        increase_energy(&mut grid);

        loop {
            let flashed_points = do_flashes(&mut grid);

            // println!("flashed_points: {:?}", flashed_points);
            // println!("");
            // print_grid(&grid);

            if flashed_points.len() == 0 {
                break;
            }

            num_flashes += flashed_points.len();

            // increase neigbor energy and check flashes again
            for p in flashed_points {
                for n in get_neighbors(&grid, p.x, p.y) {
                    if n.flashed {
                        continue;
                    }
                    grid[n.y][n.x].energy += 1;
                    // println!("n: {:?}", n);
                    print_grid(&grid);
                }
            }

            // println!("");
            // print_grid(&grid);
        }

        println!("\nend of step {}:", step);
        print_grid(&grid);

        reset_flashed(&mut grid);
    }

    println!("{:?}", num_flashes);
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
    energy: u32,
    flashed: bool,
}

fn print_grid(grid: &Vec<Vec<Point>>) {
    for row in grid {
        for p in row {
            print!("{}", p.energy)
        }
        println!();
    }
}

fn increase_energy(grid: &mut Vec<Vec<Point>>) {
    for row in grid.iter_mut() {
        for point in row.iter_mut() {
            point.energy += 1;
        }
    }
}

fn reset_flashed(grid: &mut Vec<Vec<Point>>) {
    for row in grid.iter_mut() {
        for point in row.iter_mut() {
            point.flashed = false;
        }
    }
}

fn do_flashes(grid: &mut Vec<Vec<Point>>) -> Vec<Point> {
    let mut flashed_points = Vec::new();
    for row in grid.iter_mut() {
        for point in row.iter_mut() {
            if point.energy > 9 {
                point.energy = 0;
                if !point.flashed {
                    point.flashed = true;
                    flashed_points.push(point.clone());
                }
            }
        }
    }
    flashed_points
}

fn get_neighbors(grid: &Vec<Vec<Point>>, x: usize, y: usize) -> Vec<Point> {
    let max_width = grid[0].len();
    let max_height = grid.len();

    let mut neighbors = Vec::new();
    // up left
    if x > 0 && y > 0 {
        neighbors.push(grid[y - 1][x - 1].clone());
    }
    // up
    if y > 0 {
        neighbors.push(grid[y - 1][x].clone());
    }
    // up right
    if x < max_width - 1 && y > 0 {
        neighbors.push(grid[y - 1][x + 1].clone());
    }
    // left
    if x > 0 {
        neighbors.push(grid[y][x - 1].clone());
    }
    // right
    if x < max_width - 1 {
        neighbors.push(grid[y][x + 1].clone());
    }
    // down left
    if x > 0 && y < max_height - 1 {
        neighbors.push(grid[y + 1][x - 1].clone());
    }
    // down
    if y < max_height - 1 {
        neighbors.push(grid[y + 1][x].clone());
    }
    // down right
    if x < max_width - 1 && y < max_height - 1 {
        neighbors.push(grid[y + 1][x + 1].clone());
    }
    return neighbors;
}
