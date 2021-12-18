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
    println!("final read {:?}", parse_packet_hex(&contents.trim()));

    // println!("final read {:?}", parse_packet_hex(filename.trim()));
}
fn parse_packet_hex(s: &str) -> (usize, usize) {
    let input: String = s.chars().map(to_binary).collect();
    parse_packet_string(&input)
}

fn parse_packet_string(s: &str) -> (usize, usize) {
    let bits: Vec<char> = s.chars().collect();
    parse_packet(&bits[..])
}

const SUM: usize = 0;
const PRODUCT: usize = 1;
const MIN: usize = 2;
const MAX: usize = 3;
const LITERAL: usize = 4;
const GT: usize = 5;
const LT: usize = 6;
const EQ: usize = 7;

fn parse_packet(bits: &[char]) -> (usize, usize) {
    print!("parsing packet ");
    print_str(bits);

    let mut len_read = 6;
    println!("reading 6 to get packet header");
    let v = to_decimal(&bits[0..3]);
    println!("\t version = {}", v);
    let t = to_decimal(&bits[3..6]);
    println!("\t type = {}", t);

    if t == LITERAL {
        let (num_read, value) = parse_literal_data(&bits[6..]);
        len_read += num_read;
        println!("literal read total {}", len_read);
        return (len_read, value);
    }

    let (num_read, value) = parse_operator_data(&bits[6..], t);
    len_read += num_read;
    println!("operator read total {}", len_read);
    return (len_read, value);
}

fn print_str(bits: &[char]) {
    for b in bits {
        print!("{}", b);
    }
    println!()
}

fn parse_literal_data(bits: &[char]) -> (usize, usize) {
    print!("parsing literal ");
    print_str(bits);
    let mut i = 0;
    let mut data = Vec::new();
    loop {
        println!("reading 5 to get data chunk");
        data.extend_from_slice(&bits[i + 1..i + 5]);
        if bits[i] == '0' {
            i += 5;
            break;
        } else {
            i += 5;
        }
    }
    print!("literal {}", to_decimal(&data[..]));
    print_str(&data);
    return (i, to_decimal(&data[..]));
}

fn parse_operator_data(bits: &[char], op: usize) -> (usize, usize) {
    print!("parsing operator ");
    print_str(&bits);
    println!("reading 1 to get operator size type");

    let mut vals = Vec::new();

    if bits[0] == '0' {
        // read size as 15 bits
        let size = to_decimal(&bits[1..15 + 1]);
        println!("reading 15 to get operator size from bits");
        println!("\thas size {}", size);
        // parse packet based on size return read size
        let bits = &bits[16..];
        let mut used = 0;
        while used < size {
            let (num_read, value) = parse_packet(&bits[used..size]);
            used += num_read;
            vals.push(value);
            println!("used {} of {}", used, size);
        }
        return (16 + used, do_op(vals, op));
    }

    // representing the number of sub packets
    let num_packets = to_decimal(&bits[1..11 + 1]);
    println!("\thas num packets {}", num_packets);
    let mut start = 12;
    for i in 1..num_packets + 1 {
        println!("parsing packet #{}", i);
        // figure out where the previous packet ended to set start just after that
        let (num_read, value) = parse_packet(&bits[start..]);
        start += num_read;
        vals.push(value);
        println!("next start point {}", start);
    }

    return (start, do_op(vals, op));
}

fn do_op(vals: Vec<usize>, op: usize) -> usize {
    match op {
        SUM => vals.iter().sum(),
        PRODUCT => {
            let mut r = 1;
            for v in vals {
                r *= v;
            }
            r
        }
        MIN => *vals.iter().min().unwrap(),
        MAX => *vals.iter().max().unwrap(),
        GT => {
            if vals[0] > vals[1] {
                1
            } else {
                0
            }
        }
        LT => {
            if vals[0] < vals[1] {
                1
            } else {
                0
            }
        }
        EQ => {
            if vals[0] == vals[1] {
                1
            } else {
                0
            }
        }
        _ => std::panic::panic_any(format!("unimplemented op: {}", op)),
    }
}

fn to_decimal(bits: &[char]) -> usize {
    let mut result = 0;
    for bit in bits.iter() {
        result = result << 1;
        if *bit == '1' {
            result += 1
        }
    }
    result
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => std::panic::panic_any("unexpected hex value"),
    }
}
