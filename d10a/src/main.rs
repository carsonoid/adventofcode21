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

    let program: Vec<&str> = Vec::new();

    let mut first_chars = Vec::new();

    for line in contents.trim().split("\n") {
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
                        println!("SKIP: {}: expected {} got {}", line, want, c);
                        first_chars.push(c);
                        break;
                    }
                    next.pop();
                }
            }
        }
    }

    println!("{:?}", first_chars);

    let mut answer = 0;
    let points_map = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    for c in first_chars {
        answer += points_map[&c];
    }
    println!("answer: {}", answer);
}
