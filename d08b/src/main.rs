use std::collections::HashMap;
use std::collections::HashSet;
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

    // Vec<(Vec<patterns>, Vec<displays>)>
    let mut input: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    for line in contents.trim().split("\n") {
        let parts: Vec<&str> = line.split("|").collect();
        println!("{:?}", line);
        let patterns: Vec<&str> = parts[0].trim().split_whitespace().collect();
        let displays: Vec<&str> = parts[1].trim().split_whitespace().collect();
        input.push((patterns, displays));
    }
    println!("{:?}", input);

    let mut sum = 0;
    for set in input {
        println!("set: {:?}", set);
        // figure out patterns
        let mut key_segments: HashMap<char, HashSet<char>> = HashMap::new();
        let mut key_segments_069: Vec<HashSet<char>> = Vec::new();
        let mut key_segments_235: Vec<HashSet<char>> = Vec::new();
        for pattern in set.0 {
            println!("pattern: {:?}", pattern);
            match pattern.len() {
                2 => {
                    println!("Input is showing 1");
                    key_segments.insert('1', pattern.chars().collect());
                }
                4 => {
                    println!("Input is showing 4");
                    key_segments.insert('4', pattern.chars().collect());
                }
                3 => {
                    println!("Input is showing 7");
                    key_segments.insert('7', pattern.chars().collect());
                }
                5 => {
                    println!("Input is showing 2,3,5");
                    key_segments_235.push(pattern.chars().collect());
                }
                6 => {
                    println!("Input is showing 0,6,9");
                    key_segments_069.push(pattern.chars().collect());
                }
                7 => {
                    println!("Input is showing 8");
                    key_segments.insert('8', pattern.chars().collect());
                }
                _ => std::panic::panic_any("unexpected pattern length"),
            }
        }
        println!("segments: {:?}", key_segments);
        println!("segments: {:?}", key_segments_069);

        // figure out 'a' using 1 and 7
        let a = (&key_segments[&'7'] - &key_segments[&'1'])
            .drain()
            .collect::<Vec<char>>()[0];
        // let a: char = aset[0];
        println!("a  {:?}", a);

        // figure out 'bd' using 4 and 1
        let bd = &key_segments[&'4'] - &key_segments[&'1'];
        println!("bd {:?}", bd);

        // figure out b and d using one of 0,6,9 and bd
        let mut d: char = ' ';
        for set in &key_segments_069 {
            let mut choices = &bd - &set;
            if choices.len() == 1 {
                d = choices.drain().collect::<Vec<char>>()[0];
            }
        }
        println!("d  {:?}", d);
        let dset = HashSet::from([d]);
        let b = (&bd - &dset).drain().collect::<Vec<char>>()[0];
        println!("b  {:?}", b);

        // figure out 0 from 069 and d
        let mut found_at = 0;
        for (i, set) in key_segments_069.iter().enumerate() {
            if !set.contains(&d) {
                found_at = i;
                key_segments.insert('0', set.clone());
            }
        }
        // remove 0 from 069
        key_segments_069.remove(found_at);
        println!("0  {:?}", key_segments[&'0']);

        // figure out 6 and 9 from 1 and 6,9
        for set in key_segments_069.iter() {
            if (set - &key_segments[&'1']).len() == 4 {
                key_segments.insert('9', set.clone());
            } else {
                key_segments.insert('6', set.clone());
            }
        }
        // remove 0 from 069
        println!("6  {:?}", key_segments[&'6']);
        println!("9  {:?}", key_segments[&'9']);

        println!("{:?}", key_segments);
        // we now have 0,1,4,6,7,8,9
        // missing 2,3,5

        // figure out 3 using 1 and 235
        let mut found_at = 0;
        for (i, set) in key_segments_235.iter().enumerate() {
            println!("{:?}", (set - &key_segments[&'1']));
            if (set - &key_segments[&'1']).len() == 3 {
                key_segments.insert('3', set.clone());
                found_at = i;
            }
        }
        key_segments_235.remove(found_at);
        println!("3  {:?}", key_segments[&'3']);

        // figure out 5 and to using 25 and d
        for set in key_segments_235.iter() {
            println!("{:?}", (set - &key_segments[&'1']));
            if (set - &key_segments[&'1']).contains(&b) {
                key_segments.insert('5', set.clone());
                println!("{:?} is 5", set);
            } else {
                println!("{:?} is 2", set);
                key_segments.insert('2', set.clone());
            }
        }

        // we now have it all!
        println!("Got it:");
        for (display, set) in &key_segments {
            println!("{} {:?}", display, set);
        }

        // figure out display
        let mut number: String = String::new();
        for digit in set.1 {
            let key: HashSet<char> = digit.chars().collect();
            for (k, v) in &key_segments {
                if v == &key {
                    number.push(*k);
                }
            }
        }
        println!("{}", number);

        sum = sum + number.parse::<i32>().expect("expected number");
    }

    println!("Total {}", sum);
}
