mod graph;

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

    let mut grid: Vec<Vec<Point>> = Vec::new();

    for (y, line) in contents.trim().split("\n").enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            // println!("{}", c);
            let e = c as u32 - '0' as u32;
            row.push(Point {
                x: x,
                y: y,
                risk: e,
            });
        }
        grid.push(row);
    }

    // build graph
    let mut graph = graph::Graph::new();

    for row in &grid {
        for p in row {
            graph.add_node(graph::Node {
                id: p.name().to_string(),
            });
        }
    }
    for row in &grid {
        for p in row {
            for n in get_neighbors(&grid, p.x, p.y) {
                graph.add_edge(&p.name(), &n.name(), n.risk);
            }
        }
    }
    println!("{:?}", graph.nodes);

    // find path
    let start = grid[0][0].clone();
    let end = grid[grid.len() - 1][grid[0].len() - 1].clone();

    graph::get_shortest(&start.name(), &end.name(), &graph);
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
    risk: u32,
}

impl Point {
    // fn coords(&self) -> (usize, usize) {
    //     return (self.x, self.y);
    // }
    fn name(&self) -> String {
        return format!("{},{}", self.x, self.y);
    }
}

// fn print_path(path: &Vec<Point>, grid: &Vec<Vec<Point>>) {
//     for p in path {
//         print!("->{:?}", p.coords())
//     }
//     println!();

//     let mut g = grid.clone();
//     reset_grid(&mut g);
//     for p in path {
//         g[p.y][p.x].visited = true;
//     }
//     print_grid(&g);
// }

// fn print_grid(grid: &Vec<Vec<Point>>) {
//     for row in grid {
//         for p in row {
//             if p.visited {
//                 print!("X")
//             } else {
//                 print!("{}", p.risk)
//             }
//         }
//         println!();
//     }
// }

// fn reset_grid(grid: &mut Vec<Vec<Point>>) {
//     for row in grid.iter_mut() {
//         for point in row.iter_mut() {
//             point.visited = false;
//         }
//     }
// }

fn get_neighbors(grid: &Vec<Vec<Point>>, x: usize, y: usize) -> Vec<Point> {
    let max_width = grid[0].len();
    let max_height = grid.len();

    let mut neighbors = Vec::new();

    // // up left
    // if x > 0 && y > 0 {
    //     neighbors.push(grid[y - 1][x - 1].clone());
    // }

    // up
    if y > 0 {
        neighbors.push(grid[y - 1][x].clone());
    }

    // // up right
    // if x < max_width - 1 && y > 0 {
    //     neighbors.push(grid[y - 1][x + 1].clone());
    // }

    // left
    if x > 0 {
        neighbors.push(grid[y][x - 1].clone());
    }

    // right
    if x < max_width - 1 {
        neighbors.push(grid[y][x + 1].clone());
    }

    // // down left
    // if x > 0 && y < max_height - 1 {
    //     neighbors.push(grid[y + 1][x - 1].clone());
    // }
    // down

    if y < max_height - 1 {
        neighbors.push(grid[y + 1][x].clone());
    }

    // // down right
    // if x < max_width - 1 && y < max_height - 1 {
    //     neighbors.push(grid[y + 1][x + 1].clone());
    // }

    return neighbors;
}
