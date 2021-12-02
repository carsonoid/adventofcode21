use std::fs;

fn main() {
    println!("Reading file");

    let contents = fs::read_to_string("in.txt").expect("error reading file");

    let mut last: u32 = 0;
    let mut increases: u32 = 0;
    for line in contents.split("\n") {
        println!("{}", line);
        if line == "" {
            continue;
        }

        let cur: u32 = line.trim().parse().expect("wanted a number");

        if cur > last {
            increases += 1;
        }
        last = cur;
    }

    println!("num increases: {}", increases - 1);
}
