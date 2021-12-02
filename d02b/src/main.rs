use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    if filename == "" {
        println!("filename arg is required");
    }

    println!("Reading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("error reading file");

    let mut instructions: Vec<(&str, u32)> = Vec::new();
    for line in contents.split("\n") {
        println!("{}", line);
        if line == "" {
            continue;
        }

        let parts = line.split(" ").collect::<Vec<&str>>();
        println!("{:?}", parts);
        let direction = parts[0];
        let distance = parts[1].parse().expect("wanted a number");

        instructions.push((direction, distance))
    }

    println!("{:?}", instructions);

    let mut aim = 0;
    let mut depth = 0;
    let mut h_pos = 0;

    for (direction, distance) in instructions {
        if direction == "up" {
            aim -= distance;
        }
        if direction == "down" {
            aim += distance;
        }
        if direction == "forward" {
            h_pos += distance;
            depth += distance * aim;
        }
    }

    println!("depth: {}, h_pos: {}", depth, h_pos);
    println!("answer: {}", depth * h_pos);
}
