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

    println!("{:?}", parts);

    let mut replacements: HashMap<String, String> = HashMap::new();

    for line in parts[1].trim().split("\n") {
        let rparts: Vec<&str> = line.trim().split(" -> ").collect();
        replacements.insert(rparts[0].to_string(), rparts[1].to_string());
    }

    println!("{:?}", replacements);

    let mut pairs = get_pairs(parts[0]);
    println!("{:?}", pairs);

    let mut new_line: String = "".to_string();
    for step in 1..11 {
        new_line = "".to_string();
        for pair in &pairs {
            if replacements.contains_key(pair) {
                let chars: Vec<char> = pair.chars().collect();
                let new_part = chars[0].to_string() + &replacements[pair].to_string();
                new_line += &new_part;
            } else {
                new_line += pair;
            }
            // println!("{:?}", new_line);
        }
        let last_char = pairs.last().unwrap().chars().last().unwrap();
        new_line += &last_char.to_string();
        println!("step {}: {:?}", step, new_line);

        pairs = get_pairs(&new_line);
    }

    println!("{}", get_answer(&new_line));
}

fn get_answer(source: &str) -> usize {
    let mut chars = source.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    println!("chars: {:?}", chars);

    let mut counts: Vec<(char, usize)> = Vec::new();
    for c in chars {
        counts.push((c, source.matches(c).count()))
    }
    counts.sort_by(|a, b| a.1.cmp(&b.1));
    counts.reverse();
    println!("counts: {:?}", counts);

    counts[0].1 - counts.last().unwrap().1
}

fn get_pairs(source: &str) -> Vec<String> {
    let mut pairs: Vec<String> = Vec::new();

    let mut next: String = "".to_string();
    for c in source.chars() {
        next = next + &c.to_string().clone();
        if next.len() == 2 {
            pairs.push(next.clone());
            next = next[1..].to_string();
        }
    }

    pairs
}
