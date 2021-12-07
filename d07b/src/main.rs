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

    let input: Vec<u64> = contents
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let max = input.iter().max().unwrap();
    let min = input.iter().min().unwrap();
    println!("Checking {:?}, {}, {}", input, min, max);

    let mut answer_min = 0;
    let mut answer = 0;
    for n in *min..*max + 1 {
        let mut sum = 0;
        // println!("Checking {}", n);
        for crab in &input {
            let mut diff = 0;
            if crab < &n {
                diff = n - crab;
            } else {
                diff = crab - n;
            }
            for (x, i) in (0..diff).enumerate() {
                sum += i + 1;
            }
        }

        // println!("  fuel = {}", sum);
        if answer_min == 0 || sum < answer_min {
            answer_min = sum;
            answer = n;
        }
    }

    println!("Best Position: {} with fuel {}", answer, answer_min)
}
