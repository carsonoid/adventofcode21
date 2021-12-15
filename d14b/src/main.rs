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
    let num_steps: usize = args[2].parse().unwrap();
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

    let mut pairs: HashMap<String, usize> = HashMap::new();

    let mut next: String = "".to_string();
    for c in parts[0].chars() {
        next = next + &c.to_string().clone();
        if next.len() == 2 {
            let p = next.clone();
            let e = pairs.entry(p).or_default();
            *e += 1;
            next = next[1..].to_string();
        }
    }

    println!("{:?}", pairs);

    for step in 1..num_steps + 1 {
        println!("step {:?}", step);
        let mut new_pairs: HashMap<String, usize> = HashMap::new();

        for (pair, count) in &pairs {
            if replacements.contains_key(pair) {
                println!("{} {}", pair, count);
                let chars = pair.chars().collect::<Vec<char>>();
                let p1 = chars[0];
                let p2 = chars[1];
                // NN:5 -> NC:5 & CN:5
                let e1 = p1.to_string() + replacements.get(pair).unwrap();
                let e2 = replacements.get(pair).unwrap().to_string() + &p2.to_string();
                println!("{} {}", e1, e2);

                // add to new pairs based on count
                let e1 = new_pairs.entry(e1).or_default();
                *e1 += count;

                let e2 = new_pairs.entry(e2).or_default();
                *e2 += count;
            }
        }

        println!("{:?}", new_pairs);
        pairs = new_pairs;
    }

    // add last char as a "pair" so final counts aren't off by one
    pairs.insert(parts[0].chars().last().unwrap().to_string(), 1);

    println!("{}", get_answer(&pairs));
}

fn get_answer(pairs: &HashMap<String, usize>) -> usize {
    let mut counts_map: HashMap<char, usize> = HashMap::new();
    for (pair, count) in pairs {
        println!("{:?} has {}", pair, count);
        for c in pair.chars() {
            let e = counts_map.entry(c).or_default();
            *e += count;
        }
    }

    println!("counts_map: {:?}", counts_map);

    let mut counts = Vec::new();
    for (c, size) in counts_map {
        println!("{:?} {:?} {}", c, size, size / 2);
        counts.push((c, size / 2));
    }

    counts.sort_by(|a, b| a.1.cmp(&b.1));
    counts.reverse();

    println!("counts: {:?}", counts);

    counts[0].1 - counts.last().unwrap().1
}
