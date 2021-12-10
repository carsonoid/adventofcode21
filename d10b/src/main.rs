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

    let mut totals = Vec::new();
    let points_map = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    for line in contents.trim().split("\n") {
        let mut invalid = false;

        let mut next = Vec::new();

        for c in line.chars() {
            match c {
                '(' => next.push(')'),
                '[' => next.push(']'),
                '{' => next.push('}'),
                '<' => next.push('>'),
                _ => {
                    let want = next[next.len() - 1];
                    if want != c {
                        // println!("SKIP: {}: expected {} got {}", line, want, c);
                        invalid = true;
                        break;
                    }
                    next.pop();
                }
            }
        }

        if invalid {
            continue;
        }

        next.reverse();
        println!("next: {:?}", next);

        let mut val: i64 = 0;
        for c in next {
            val = val * 5;
            val = val + points_map[&c];
        }
        println!("next: {:?}", val);
        totals.push(val);
    }

    println!("totals: {:?}", totals);

    totals.sort();
    println!("totals: {:?}", totals);

    println!("answer: {:?}", totals[(totals.len() / 2)]);
}
