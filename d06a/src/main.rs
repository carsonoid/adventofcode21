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

    let input: Vec<u32> = contents
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut counts: HashMap<u32, u32> = HashMap::new();
    for num_days in input {
        if counts.contains_key(&num_days) {
            counts.insert(num_days, counts[&num_days] + 1);
        } else {
            counts.insert(num_days, 1);
        }
    }

    println!("Initial State: {:?}", counts);

    for day in 1..81 {
        println!("day {:?}", day);

        let mut new_counts: HashMap<u32, u32> = HashMap::new();

        for (k, v) in &counts {
            if *k == 0 {
                // add to 6 day count
                if new_counts.contains_key(&6) {
                    new_counts.insert(6, v + new_counts[&6]);
                } else {
                    new_counts.insert(6, *v);
                }
                // add to 8 day count
                if new_counts.contains_key(&8) {
                    new_counts.insert(8, v + new_counts[&8]);
                } else {
                    new_counts.insert(8, *v);
                }
            } else {
                // add to one less day count
                let next = *k - 1;
                if new_counts.contains_key(&next) {
                    new_counts.insert(next, v + new_counts[&next]);
                } else {
                    new_counts.insert(next, *v);
                }
            }
        }

        counts = new_counts;
        println!("  New State: {:?}", counts);
    }

    let mut sum = 0;
    for (_, v) in &counts {
        sum += v;
    }

    println!("Sum: {:?}", sum);
}
