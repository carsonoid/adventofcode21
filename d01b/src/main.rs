use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Reading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("error reading file");

    let mut nums: Vec<u32> = Vec::new();
    for line in contents.split("\n") {
        println!("{}", line);
        if line == "" {
            continue;
        }

        let num: u32 = line.trim().parse().expect("wanted a number");
        nums.push(num)
    }

    let mut last: u32 = 0;
    let mut increases: u32 = 0;

    for (i, num) in nums.iter().enumerate() {
        let mut cur = *num;

        cur += nums.get(i + 1).ok_or(0).unwrap_or(&0);
        cur += nums.get(i + 2).ok_or(0).unwrap_or(&0);

        println!("{} = {}", i, cur);

        if cur > last {
            increases += 1;
        }

        last = cur;
    }

    println!("num increases: {}", increases - 1);
}
