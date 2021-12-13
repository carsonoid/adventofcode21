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
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let mut grid: HashMap<(u32, u32), bool> = HashMap::new();
    for d in parts[0].split("\n") {
        let p: Vec<&str> = d.split(",").collect();
        let x = p[0].parse().unwrap();
        let y = p[1].parse().unwrap();
        grid.insert((x, y), true);
    }

    let mut max_y = 0;
    let mut max_x = 0;
    for k in grid.keys() {
        if k.0 > max_x {
            max_x = k.0;
        }
        if k.1 > max_y {
            max_y = k.1;
        }
    }

    let mut folds: Vec<(char, u32)> = Vec::new();

    for d in parts[1].trim().split("\n") {
        let fold_parts: Vec<&str> = d.split(" ").collect::<Vec<&str>>()[2].split("=").collect();
        println!("{:?}", fold_parts);
        folds.push((
            fold_parts[0].chars().collect::<Vec<char>>()[0],
            fold_parts[1].parse().unwrap(),
        ))
    }

    println!("{:?}", folds);

    print_grid(&grid, max_x, max_y);

    // do folds
    for fold in folds {
        let mut new_map = HashMap::new();
        if fold.0 == 'y' {
            max_y = fold.1 - 1;
            println!();
            println!("folding on y at {}", fold.1);
            for e in grid.keys() {
                if e.1 > fold.1 {
                    let y_new = fold.1 - (e.1 - fold.1);
                    println!("{:?} moves to on y at {}", e, y_new);
                    new_map.insert((e.0, y_new), true);
                } else {
                    new_map.insert((e.0, e.1), true);
                }
            }
        } else {
            max_x = fold.1 - 1;
            println!();
            println!("folding on x at {}", fold.1);
            for e in grid.keys() {
                if e.0 > fold.1 {
                    let x_new = fold.1 - (e.0 - fold.1);
                    println!("{:?} moves to on y at {}", e, x_new);
                    new_map.insert((x_new, e.1), true);
                } else {
                    new_map.insert((e.0, e.1), true);
                }
            }
        }
        grid = new_map;
        println!();
        print_grid(&grid, max_x, max_y);
        println!("{} dots", grid.len());
    }
}

fn print_grid(grid: &HashMap<(u32, u32), bool>, max_x: u32, max_y: u32) {
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if grid.contains_key(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}
