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

    let mut input: Vec<Vec<&str>> = Vec::new();
    for line in contents.trim().split("\n") {
        let parts: Vec<&str> = line.split("|").collect();
        println!("{:?}", line);
        let displays: Vec<&str> = parts[1].trim().split_whitespace().collect();
        input.push(displays);
    }
    println!("{:?}", input);

    let mut sum = 0;
    let check_counts: Vec<usize> = vec![2, 4, 3, 7];
    for row in input {
        println!("row: {:?}", row);
        for display in row {
            if check_counts.contains(&display.len()) {
                println!("  display: {:?} {}", display, display.len());
                sum = sum + 1;
            }
        }
        println!("total: {:?}\n", sum);
    }
    println!("total: {:?}", sum);
}
