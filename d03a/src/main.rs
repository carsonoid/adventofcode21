use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    if filename == "" {
        println!("filename arg is required");
    }

    println!("Reading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("error reading file");

    let mut report: Vec<Vec<bool>> = Vec::new();
    for line in contents.split("\n") {
        println!("{}", line);
        if line == "" {
            continue;
        }

        let mut instr: Vec<bool> = Vec::new();
        for c in line.chars() {
            if c == '0' {
                instr.push(false)
            } else {
                instr.push(true)
            }
        }

        report.push(instr);
    }
    println!("{:?}", report);

    let mut gamma_rate: String = String::new();
    let mut epsilon_rate: String = String::new();

    for pos in 0..12 {
        let mut count_ones = 0;
        let mut count_zeroes = 0;
        for line in &report {
            if line[pos] == true {
                count_ones += 1;
            } else {
                count_zeroes += 1;
            }
        }
        if count_ones > count_zeroes {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
            println!("{}: most: 1, least: 0", pos)
        } else {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
            println!("{}: most: 0, least: 1", pos)
        }
    }

    println!("gamma: {}, epsilon: {}", gamma_rate, epsilon_rate);
}
