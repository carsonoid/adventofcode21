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

    let mut map = HashMap::new();
    for line in contents.trim().split("\n") {
        let parts: Vec<&str> = line.split("-").collect();
        let to = parts[0];
        let from = parts[1];

        // first direction
        add_path(&mut map, to, from);
        add_path(&mut map, from, to);
    }

    // put end in with no back paths
    map.insert(
        "end".to_string(),
        Node {
            paths: Vec::new(),
            is_small: true,
        },
    );

    println!("map: {:?}\n\n", map);

    // figure out small caves from map
    let mut small_caves: Vec<String> = vec!["".to_string()];
    for (k, v) in &map {
        if v.is_small && k != "start" && k != "end" {
            small_caves.push(k.clone());
        }
    }
    println!("small_caves: {:?}", small_caves);

    // return;

    println!("START");

    let mut count: u64 = 0;
    let mut followed_path = Vec::new();

    // get all paths with revisit
    for cave in small_caves {
        // println!("CAN revisit '{}'", cave);
        do_next(&"start", &mut map, &mut followed_path, &mut count, &cave);
    }

    println!("paths {}", count);
}

fn add_path(map: &mut HashMap<String, Node>, from: &str, to: &str) {
    if to == "start" || from == "end" {
        return;
    }

    map.entry(from.to_string())
        .or_insert(Node {
            paths: Vec::new(),
            is_small: from.chars().all(|c| c.is_ascii_lowercase()),
        })
        .paths
        .push(to.to_string());
}

fn do_next(
    current_name: &str,
    map: &mut HashMap<String, Node>,
    followed_path: &mut Vec<String>,
    count: &mut u64,
    revisit: &str,
) {
    followed_path.push(current_name.to_string());

    if current_name == "end" {
        if revisit == "" {
            println!("END {:?}", followed_path);
            *count = *count + 1;
        } else {
            let num_visits = followed_path.iter().filter(|&x| *x == revisit).count();
            if num_visits == 2 {
                println!("END {} {:?}", revisit, followed_path);
                *count = *count + 1;
            }
        }
        return;
    }

    // println!("{:?}", followed_path);

    let paths = map.get_mut(current_name).unwrap().paths.clone();

    for next_name in paths {
        let next = map.get(&next_name).unwrap();

        // println!("{}: {:?}", next_name, next);
        let num_visits = followed_path.iter().filter(|&x| *x == next_name).count();

        if next_name != "end" && next.is_small {
            println!("  CHECK {} num:{}", next_name, num_visits,);

            if next_name == revisit {
                if num_visits == 2 {
                    println!("CANNOT visit {} more than twice", next_name);
                    continue;
                }
            } else if num_visits == 1 {
                // println!("CANNOT revisit {}", next_name);
                continue;
            }
        }

        // go to next and test again
        println!("MOVE {} -> {}", current_name, next_name);
        do_next(&next_name, map, &mut followed_path.clone(), count, revisit);
    }
}

#[derive(Debug)]
struct Node {
    paths: Vec<String>,
    is_small: bool,
}
