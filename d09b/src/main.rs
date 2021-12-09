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
            // println!("{}", c);
            row.push(c as u32 - '0' as u32);
        }
        grid.push(row);
    }

    // for row in &grid {
    //     println!("{:?}", row);
    // }

    let mut low_points: Vec<Point> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let neighbors = get_neighbors(&grid, x, y);
            let mut is_low = true;
            for n in neighbors {
                if n.height <= *height {
                    is_low = false;
                    break;
                }
            }
            if is_low {
                // println!("{},{} is low point of {}", x, y, height);
                low_points.push(Point {
                    x: x,
                    y: y,
                    height: *height,
                });
            }
        }
    }

    // testing
    // low_points.remove(0);

    let mut sizes = Vec::new();
    for point in low_points {
        println!("basin at {:?}", point);

        let mut visited: Vec<Point> = Vec::new();
        get_size(&grid, point.x, point.y, &mut visited);
        println!("  bsize {}", visited.len());
        sizes.push(visited.len());
        // break;
    }

    sizes.sort();
    let last3 = sizes.as_slice()[sizes.len() - 3..].to_vec();

    println!("{:?}", last3);

    let mut answer = 1;
    for s in last3 {
        answer = answer * s;
    }
    println!("{:?}", answer);
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
    height: u32,
}

fn get_size(grid: &Vec<Vec<u32>>, x: usize, y: usize, visited: &mut Vec<Point>) {
    println!("checking size at: {}, {}", x, y);
    let neighbors: Vec<Point> = get_neighbors(&grid, x, y)
        .into_iter()
        .filter(|p| !visited.contains(p) && p.height < 9)
        .collect();
    println!("  neighbors: {:?}", neighbors);

    visited.push(Point {
        x: x,
        y: y,
        height: grid[y][x],
    });

    for n in neighbors {
        if visited.contains(&n) {
            continue;
        }
        get_size(grid, n.x, n.y, visited);
        println!("visited: {:?}", visited);
    }
}

fn get_neighbors(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<Point> {
    let max_width = grid[0].len();
    let max_height = grid.len();

    let mut neighbors = Vec::new();
    // left
    if x > 0 {
        neighbors.push(Point {
            x: x - 1,
            y: y,
            height: grid[y][x - 1],
        });
    }
    // right
    if x < max_width - 1 {
        neighbors.push(Point {
            x: x + 1,
            y: y,
            height: grid[y][x + 1],
        });
    }
    // up
    if y > 0 {
        neighbors.push(Point {
            x: x,
            y: y - 1,
            height: grid[y - 1][x],
        });
    }
    // down
    if y < max_height - 1 {
        neighbors.push(Point {
            x: x,
            y: y + 1,
            height: grid[y + 1][x],
        });
    }

    return neighbors;
}
