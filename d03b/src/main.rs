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
        // println!("{}", line);
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
    // println!("{:?}", report);
    let mut report2 = report.clone();

    for pos in 0..12 {
        println!("{}", pos);
        let mut most_common = true;
        let mut count_ones = 0;
        let mut count_zeroes = 0;
        for line in &report {
            if line[pos] == true {
                count_ones += 1;
            } else {
                count_zeroes += 1;
            }
        }
        if count_ones < count_zeroes {
            most_common = false;
        }

        let mut new_report: Vec<Vec<bool>> = Vec::new();
        for line in &report {
            if line[pos] == most_common {
                new_report.push(line.clone());
            }
        }
        report = new_report;

        if report.len() == 1 {
            print!("oxygen generator: {:?} = ", report[0]);
            for v in &report[0] {
                if *v {
                    print!("1")
                } else {
                    print!("0")
                }
            }
            println!();
            break;
        }
        if pos == 11 {
            println!("Got through without getting to one entry")
        }
    }

    for pos in 0..12 {
        let mut least_common = false;
        let mut count_ones = 0;
        let mut count_zeroes = 0;
        for line in &report2 {
            if line[pos] == true {
                count_ones += 1;
            } else {
                count_zeroes += 1;
            }
        }
        if count_ones < count_zeroes {
            least_common = true;
        }

        // println!("{}, {}", pos, least_common);

        let mut new_report: Vec<Vec<bool>> = Vec::new();
        for line in &report2 {
            if line[pos] == least_common {
                new_report.push(line.clone());
            }
        }
        report2 = new_report;
        // println!("{:?}", report2);

        if report2.len() == 1 {
            print!("c02 scrubber: {:?} = ", report2[0]);
            for v in &report2[0] {
                if *v {
                    print!("1")
                } else {
                    print!("0")
                }
            }
            println!();
            break;
        }
    }
}
