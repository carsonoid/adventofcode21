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
        let k = parts[0].to_string();
        let k_big = !k.chars().all(|c| c.is_ascii_lowercase());
        let v = parts[1];
        let v_big = !v.chars().all(|c| c.is_ascii_lowercase());

        // first direction
        map.entry(k.clone())
            .or_insert(Node {
                paths: Vec::new(),
                is_big: k_big,
            })
            .paths
            .push(v.to_string());

        // second directoin
        map.entry(v.to_string())
            .or_insert(Node {
                paths: Vec::new(),
                is_big: v_big,
            })
            .paths
            .push(k.clone());
    }

    println!("{:?}\n\n", map);

    println!("START");

    let mut count: u64 = 0;
    let mut followed_path = Vec::new();

    do_next(&"start", &mut map, &mut followed_path, &mut count);

    println!("paths {}", count);
}

fn do_next(
    current_name: &str,
    map: &mut HashMap<String, Node>,
    followed_path: &mut Vec<String>,
    count: &mut u64,
) {
    followed_path.push(current_name.to_string());

    if current_name == "end" {
        println!("END {:?}", followed_path);
        *count = *count + 1;
        return;
    }

    println!("{:?}", followed_path);

    let paths = map.get_mut(current_name).unwrap().paths.clone();

    for next_name in paths {
        let next = map.get(&next_name).unwrap();
        println!(" check {} to {}: {:?}", current_name, next_name, next);

        // println!("{}: {:?}", next_name, next);
        let is_visited = followed_path.contains(&next_name);
        if !next.is_big && is_visited {
            continue;
        }

        // go to next and test again
        // println!("MOVE {} -> {}", current_name, next_name);
        do_next(&next_name, map, &mut followed_path.clone(), count);
    }
}

#[derive(Debug)]
struct Node {
    paths: Vec<String>,
    is_big: bool,
}
